use crate::{ModulesOrModuleMenus, MyError};
use actix_web::Either;
use actix_web::Responder;
use actix_web::{
    get,
    web::{Data, Json, Path},
};
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::JoinOnDsl;
use diesel::NullableExpressionMethods;
use diesel::PgExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tucan_scraper::{
    models::{Module, ModuleMenu},
    schema::{module_menu_module, module_menu_tree, module_menu_unfinished, modules_unfinished},
    tucan::Tucan,
};

// trailing slash is menu
#[get("/modules{tail:.*}")]
pub async fn get_modules<'a>(
    tucan: Data<Tucan>,
    path: Path<String>,
) -> Result<impl Responder, MyError> {
    let mut connection = tucan.pool.get().await?;

    let split_path = path.split_terminator('/').map(String::from);
    let menu_path_vec = split_path.skip(1).collect::<Vec<_>>();

    let menu_path: Vec<String>;
    let module: Option<&str>;
    if path.ends_with('/') {
        menu_path = menu_path_vec;
        module = None;
    } else {
        let tmp = menu_path_vec.split_last().unwrap();
        menu_path = tmp.1.to_vec();
        module = Some(tmp.0);
    }

    let mut node = None;
    for path_segment in menu_path {
        let the_parent = node.map(|v: ModuleMenu| v.tucan_id);

        node = Some(
            module_menu_unfinished::table
                .left_outer_join(
                    module_menu_tree::table
                        .on(module_menu_tree::child.eq(module_menu_unfinished::tucan_id)),
                )
                .select(module_menu_unfinished::all_columns)
                .filter(
                    module_menu_tree::parent
                        .nullable()
                        .is_not_distinct_from(the_parent)
                        .and(module_menu_unfinished::normalized_name.eq(path_segment)),
                )
                .load::<ModuleMenu>(&mut connection)
                .await?
                .into_iter()
                .next()
                .unwrap(),
        )
    }
    let parent = node.map(|v: ModuleMenu| v.tucan_id);

    if let Some(module) = module {
        let module_result = module_menu_module::table
            .inner_join(modules_unfinished::table)
            .select((
                modules_unfinished::tucan_id,
                modules_unfinished::tucan_last_checked,
                modules_unfinished::title,
                modules_unfinished::module_id,
                modules_unfinished::credits,
                modules_unfinished::content,
                modules_unfinished::done,
            ))
            .filter(
                module_menu_module::module_menu_id
                    .eq(parent.unwrap())
                    .and(modules_unfinished::module_id.eq(module)),
            )
            .load::<Module>(&mut connection)
            .await?
            .into_iter()
            .next()
            .unwrap();

        Ok(Either::Left(Json(module_result)))
    } else {
        let menu_result = module_menu_unfinished::table
            .left_outer_join(
                module_menu_tree::table
                    .on(module_menu_tree::child.eq(module_menu_unfinished::tucan_id)),
            )
            .select(module_menu_unfinished::all_columns)
            .filter(
                module_menu_tree::parent
                    .nullable()
                    .is_not_distinct_from(&parent),
            )
            .load::<ModuleMenu>(&mut connection)
            .await?;

        let module_result = module_menu_module::table
            .inner_join(modules_unfinished::table)
            .filter(module_menu_module::module_menu_id.nullable().eq(&parent))
            .select((
                modules_unfinished::tucan_id,
                modules_unfinished::tucan_last_checked,
                modules_unfinished::title,
                modules_unfinished::module_id,
                modules_unfinished::credits,
                modules_unfinished::content,
                modules_unfinished::done,
            ))
            .load::<Module>(&mut connection)
            .await?;

        if !menu_result.is_empty() {
            Ok(Either::Right(Json(ModulesOrModuleMenus::Menus(
                menu_result,
            ))))
        } else {
            Ok(Either::Right(Json(ModulesOrModuleMenus::Modules(
                module_result,
            ))))
        }
    }
}
