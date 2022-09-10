use std::io::{Error, ErrorKind};

use reqwest::header::HeaderValue;
use scraper::Html;
use serde::Serialize;

/**
okay this url is such a terrible mess, just store the whole last part and make parent child relationships based on that

https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N0,-N0
                                                                                                        session id      left menu id
                                                                                                                                     bscinf     always? 0
                                                                                                                                                         second+third layer?
                                                                                                                                                                fourth+fifth layer

B.Sc. Informatik (2015)  >  Zusätzliche Leistungen  >  Gesamtkatalog aller Module des Sprachenzentrums  >  Deutsch als Fremdsprache  >  UNIcert II  >  allgemeinsprachlich
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N455311578128190,-N000311,-N376333755785484,-N0,-N356174057085574,-N351244265273917

Anmeldung
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-A

B.Sc. Informatik (2015)
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N0,-N0

Validierung
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N363764281516065,-N000000000000000

Bachelor Thesis
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173440692528,-N000000000000000

Pflichtbereich
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173456785530,-N000000000000000

Wahlbereich
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173502252532,-N000000000000000

Wahlbereich > Fachprüfungen
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173534016533,-N000000000000000

Wahlbereich > Fachprüfungen > IT-Sicherheit
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173674091535,-N356173585074794

Wahlbereich > Fachprüfungen > Netze und verteilte Systeme
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173678785536,-N356173586880795

Wahlbereich > Fachprüfungen > Robotik, Computational und Computer Engineering
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173680675537,-N356173588165796

Wahlbereich > Fachprüfungen > Software Systeme und formale Grundlagen
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173682283538,-N356173589439797

Wahlbereich > Fachprüfungen > Visual & Interactive Computing
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173683996539,-N356173590893798

Wahlbereich > Fachprüfungen > Web, Wissens- und Informationsverarbeitung
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173685419540,-N356173601055801

Wahlbereich > Studienleistungen
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173862125542,-N000000000000000

Wahlbereich > Studienleistungen > Seminare
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173871470543,-N356173605606802

Wahlbereich > Studienleistungen > Praktikum in der Lehre
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173877874544,-N356173607261803

Wahlbereich > Studienleistungen > Praktika, Projektpraktika, ähnliche LV
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173881330545,-N356173609083804

Wahlbereich > Fachübergreifende Lehrveranstaltungen
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356173908703547,-N000000000000000

Wahlbereich > Fachübergreifende Lehrveranstaltungen > Module der Betriebswirtschaftslehre
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N360752852023675,-N360746547086804

Wahlbereich > Fachübergreifende Lehrveranstaltungen > Gesamtkatalog aller Module des Sprachenzentrums
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356344278774629,-N345145543638428

B.Sc. Informatik (2015)  >  Wahlbereich  >  Fachübergreifende Lehrveranstaltungen  >  Gesamtkatalog aller Module des Sprachenzentrums  >  Zentrum für Interkulturelle Kompetenz ZIKK
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356344278774629,-N350874384986426

B.Sc. Informatik (2015)  >  Wahlbereich  >  Fachübergreifende Lehrveranstaltungen  >  Gesamtkatalog aller Module des Sprachenzentrums  >  Zentrum für Interkulturelle Kompetenz ZIKK  >  Module nur für internationale Masterstudierende
https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N483115916181886,-N000311,-N376333755785484,-N0,-N356344278774629,-N360263908359080
*/
use crate::{element_by_selector, s, tucan::Tucan};

pub struct TucanUser {
    pub tucan: Tucan,
    pub(crate) session_id: String,
    pub(crate) session_nr: u64,
}

#[derive(Debug, Serialize)]
pub enum RegistrationEnum {
    Submenu(Vec<(String, String)>),
    Modules(Vec<(String, String)>), // TODO types
}

#[derive(Debug, Serialize)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub credits: Option<i64>,
    pub responsible_person: String,
    pub content: String,
}

impl TucanUser {
    pub(crate) async fn fetch_document(&self, url: &str) -> anyhow::Result<Html> {
        // TODO FIXME don't do this like that but just cache based on module id that should also be in the title on the previous page
        // maybe try the same with the navigation menus

        let normalized_url = url.to_string();
        /* if normalized_url.contains("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=") {
            normalized_url = normalized_url[0..normalized_url.rfind(",-A").unwrap()].to_string();
            //println!("normalized: {}", normalized_url);
            //println!("url       : {}", url);
        }*/

        // can't cache these as the links inside there are invalid for new sessions
        /*
        if normalized_url.contains("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=") {
            let start = normalized_url.find("ARGUMENTS=").unwrap() + "ARGUMENTS=".len();
            let end = normalized_url.find(",").unwrap() + 1;
            normalized_url = normalized_url[0..start].to_string() + &normalized_url[end..];
            //println!("normalized: {}", normalized_url);
            //println!("url       : {}", url);
        }*/

        // SELECT url, instr(url, ",-A") FROM http_cache WHERE url LIKE "%MODULEDETAILS%" ORDER BY url;
        // SELECT substr(url, 0, instr(url, ",-A")) AS b, COUNT(*) AS c FROM http_cache WHERE url LIKE "%MODULEDETAILS%" GROUP BY b ORDER BY c DESC;
        // the data at the end is random every login

        // SELECT substr(url, 0, instr(url, "PRGNAME")) FROM http_cache;

        // SELECT substr(url, instr(url, "PRGNAME"), instr(url, "&ARGUMENTS=")-instr(url, "PRGNAME")) AS a, COUNT(*) FROM http_cache GROUP BY a;

        // SELECT url FROM http_cache WHERE url LIKE "%REGISTRATION%" ORDER BY url;

        let cookie = format!("cnsc={}", self.session_id);

        let a = self.tucan.client.get(url);
        let b = a.build().unwrap();
        b.headers_mut()
            .insert("Cookie", HeaderValue::from_str(&cookie).unwrap());

        let permit = self.tucan.semaphore.acquire().await?;
        let resp = self.tucan.client.execute(b).await?.text().await?;
        drop(permit);

        let html_doc = Html::parse_document(&resp);

        if html_doc
            .select(&s("h1"))
            .any(|s| s.inner_html() == "Timeout!")
        {
            return Err(Error::new(ErrorKind::Other, "well we got a timeout here. relogin").into());
        }
        Ok(html_doc)
    }

    pub async fn module(&self, url: &str) -> anyhow::Result<Module> {
        let document = self.fetch_document(url).await?;

        let name = element_by_selector(&document, "h1").unwrap();

        let text = name.inner_html();
        let mut fs = text.split("&nbsp;");
        let module_id = fs.next().unwrap().trim();

        let module_name = fs.next().map(str::trim);

        let credits = document
            .select(&s(r#"#contentlayoutleft b"#))
            .find(|e| e.inner_html() == "Credits: ")
            .unwrap()
            .next_sibling()
            .unwrap()
            .value()
            .as_text()
            .unwrap();

        // Hinweis: In Ihrer Prüfungsordnung können abweichende Credits festgelegt sein.
        let credits = credits
            .trim()
            .strip_suffix(",0")
            .and_then(|v| v.parse::<i64>().ok());

        let responsible_person = document
            .select(&s("#dozenten"))
            .next()
            .unwrap()
            .inner_html();
        let content = document
            .select(&s("#contentlayoutleft tr.tbdata"))
            .next()
            .unwrap()
            .inner_html();

        Ok(Module {
            id: module_id.to_string(),
            name: module_name.unwrap().to_string(),
            credits,
            responsible_person,
            content,
        })
    }

    async fn traverse_module_list(&self, url: &str) -> anyhow::Result<RegistrationEnum> {
        let document = self.fetch_document(url).await?;

        // list of subcategories
        let submenu_list = element_by_selector(&document, "#contentSpacer_IE ul");

        // list of modules
        let modules_list = element_by_selector(&document, "table.tbcoursestatus");

        match (submenu_list, modules_list) {
            (_, Some(list)) => Ok(RegistrationEnum::Modules(
                list.select(&s(r#"td.tbsubhead.dl-inner a[href]"#))
                    .map(|e| {
                        (
                            e.text()
                                .map(str::to_string)
                                .reduce(|a, b| a + &b)
                                .unwrap_or_default()
                                .trim()
                                .to_string(),
                            format!(
                                "https://www.tucan.tu-darmstadt.de{}",
                                e.value().attr("href").unwrap()
                            ),
                        )
                    })
                    .collect(),
            )),
            (Some(list), None) => {
                Ok(RegistrationEnum::Submenu(
                    list.select(&s("a[href]"))
                        .map(|e| {
                            (
                                e.text()
                                    .map(str::to_string)
                                    .reduce(|a, b| a + &b)
                                    .unwrap_or_default()
                                    .trim()
                                    .to_string(),
                                format!(
                                    "https://www.tucan.tu-darmstadt.de{}",
                                    e.value().attr("href").unwrap()
                                ),
                            )
                        })
                        .collect(),
                ))

                /*
                let selector = s("a[href]");
                let iterat = list
                    .select(&selector)
                    .map(async move |b| self.handle_sublink(b).await);

                let mut futures: FuturesUnordered<_> = iterat.collect();
                while let Some(result) = futures.next().await {
                    result?;
                }
                */
            }
            _ => {
                panic!("{} {}", url, document.root_element().html())
            }
        }
    }

    pub async fn registration(&self, url: Option<String>) -> anyhow::Result<RegistrationEnum> {
        let url = url.unwrap_or(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{},-N000311,-A", self.session_nr));

        self.traverse_module_list(&url).await
    }
}
