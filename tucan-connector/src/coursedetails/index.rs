use scraper::{ElementRef, Html};
use tucant_types::{
    coursedetails::{CourseDetailsRequest, CourseDetailsResponse},
    LoginResponse, TucanError,
};

use crate::{
    common::head::{html_head, logged_in_head, logged_out_head},
    html_handler::Root,
    Tucan,
};

pub async fn coursedetails(
    tucan: &Tucan,
    login_response: &LoginResponse,
    args: CourseDetailsRequest,
) -> Result<CourseDetailsResponse, TucanError> {
    let id = login_response.id;
    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N{:015}{}", id, args.arguments);
    println!("{url}");
    // TODO FIXME generalize
    let key = format!("url.{url}");
    let content = if let Some(content) = tucan.database.get(&key).await {
        content
    } else {
        let response = tucan
            .client
            .get(url)
            .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
            .send()
            .await?
            .error_for_status()?;
        let content = response.text().await?;
        tucan.database.put(&key, &content).await;
        content
    };
    let document = Html::parse_document(&content);
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
        <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
            <head>_
    };
    let mut html_handler = html_head(html_handler);
    if html_handler.peek().is_none() {
        html_extractor::html! {
            </head>_
            <body class="timeout">
        };
        let _html_handler = html_handler;
        return Err(TucanError::Timeout);
    }
    html_extractor::html! {
            <style type="text/css">
                "Z8Nk5s0HqiFiRYeqc3zP-bPxIN31ePraM-bbLg_KfNQ"
            </style>_
            <style type="text/css">
                "3CC0xpJgjHprYY59D1krvfwrI2LSV2-OtaN3CviYnG8"
            </style>_
        </head>_
        <body class="coursedetails">_
    };
    let html_handler = if login_response.id != 1 {
        logged_in_head(html_handler, login_response.id)
    } else {
        logged_out_head(html_handler, 311)
    };
    html_extractor::html! {
        <!--"dqf58hG7HHGpXGyye2_RfFRU9OdHxiBSQr2SeCdraDU"-->_
        <script type="text/javascript">
        </script>_
        <script type="text/javascript">_trash</script>_
            <form name="courseform" action="/scripts/mgrqispi.dll" method="post">_
              <h1>"\n20-00-0040-iv\nGraphische Datenverarbeitung I\n"</h1>_
              <div class="contentlayoutleft" id="contentlayoutleft">_
                <table class="tb rw-table rw-all">_
                  <caption>"Veranstaltungsdetails "</caption>_
                  <tbody>
                    <tr>_
                      <td class="tbcontrol" colspan="3">_
                    </td>_</tr>_
                    <tr>_
                      <td class="tbdata" colspan="3">_
                        <!--"7mR3L45uIzjYs57_yUuqAgGUVvt88EQ1apLxlExwuH4"-->_
                        <p>_
                          <b>"Lehrende: "</b>
                          <span id="dozenten">"Prof. Dr. techn. Wolf Dietrich Fellner"</span>_</p>_
                        <p>
                          <b>"Veranstaltungsart:"</b>"\n\t\t\t\t\t\t\t\t                                        Integrierte Veranstaltung\n                                        "
                          <input type="hidden" name="coursetyp" value="000000000000009"></input>_</p>_
                        <p>
                          <b>"Orga-Einheit:"</b>_
                          <span name="courseOrgUnit">"FB20 Informatik"</span></p>_
                        <p>_
                          <b>"Anzeige im Stundenplan: "</b>"\n                                                                        GDV I\n                                        "
                          <input type="hidden" name="shortdescription" value="GDV I"></input>_</p>_
                        <input type="hidden" name="courselevel" value="000000000000000"></input>_
                        <p>
                          <b>"Fach:"</b>_
                          <input type="hidden" name="coursearea" value=""></input>_</p>_
                        <p>
                          <b>"Anrechenbar für:"</b>_
                          <input type="hidden" name="creditingfor" value=""></input>_</p>_
                        <p>
                          <b>"Semesterwochenstunden: "</b>"\n                                                                        4\n                                                "
                          <input type="hidden" name="sws" value="4"></input>_</p>_
                        <input type="hidden" name="credits" value="  0,0"></input>_
                        <input type="hidden" name="location" value="327576461398991"></input>_
                        <p>
                          <b>"Unterrichtssprache: "</b>_
                          <span name="courseLanguageOfInstruction">"Deutsch"</span>_
                          <input type="hidden" name="language" value="001"></input>_</p>_
                        <p>
                          <b>"Min. | Max. Teilnehmerzahl:"</b>"\n\n                              - | -\n                "
                          <input type="hidden" name="min_participantsno" value="-"></input>_
                          <input type="hidden" name="max_participantsno" value="-"></input>_</p>_
                        <!--"u8GEiL8QtgIxvCs-Vf3CkMBYw-XHp4bjwN_4-b3nrOQ"-->_

    }
    let mut description = Vec::new();
    while !html_handler.peek().unwrap().value().is_comment() {
        let child;
        (html_handler, child) = html_handler.next_any_child();
        match child.value() {
            scraper::Node::Text(text) => description.push(text.trim().to_owned()),
            scraper::Node::Element(_element) => {
                description.push(ElementRef::wrap(child).unwrap().html());
            }
            _ => panic!(),
        }
    }
    html_extractor::html! {
                        <!--"xdnrtl8EoTjGxC3Tn8ZgU7vEsjh7SULK5uyEXMTrPYw"-->_</td>_</tr>_</tbody></table>_
                <!--"BJVxG97RSYn0rh25cerEgm9r0KvMqIm48tBzBZmL9fA"-->_
                <div class="tb">_
                  <div>_
                    <div class="tbhead">"Kleingruppe(n)"</div>_
                    <div class="tbdata">"\n\t\t\t\tDie Veranstaltung ist in die folgenden Kleingruppen aufgeteilt:\n\t\t\t\t\t\t\t"</div>_</div>_
                  <ul class="dl-ul-listview">_
                    <li class="tbdata listelement">_
                      <div class="dl-inner">_
                        <p class="dl-ul-li-headline">
                          <strong>"Graphische Datenverarbeitung I Übung"</strong></p>_
                        <p>"Prof. Dr. techn. Wolf Dietrich Fellner"</p>_
                        <p>"Di, 15. Okt. 2024 [09:50]-Di, 11. Feb. 2025 [11:30]"</p>_</div>_
                      <div class="dl-link">_
                        <a href=_url class="img img_arrowLeft pageElementRight">"\n\t\t\t\t\t\t\t\t\tKleingruppe anzeigen\n\t\t\t\t\t\t\t\t"</a>_</div>_</li>_</ul>_</div>_
                <!--"0x4FAGT9tkPZPnjGhLVSIyUwzWJVg5LmPPopzaVekvg"-->_
                <!--"gjmJkszfvlTVATkzxj9UfHJAWhksvjlPhatwUMepicA"-->_
                <table class="tb rw-table">_
                  <caption>"\n                        Literatur\n                                        "</caption>_
                  <tbody>
                    <tr>_
                      <td class="tbsubhead">_
                        <span name="literatureCategory">
                          <!--"EdGg5F530M2nVMCHhp1bEr4g_yMTeijq2NDDbwiJXzI"--></span>_</td>_</tr>_</tbody></table>_
                <!--"rLgWPHovMo94GGr9fjSOcwUR-V0yqvfB-QchTzSNf04"-->_
                <!--"GwYigtfCarUUFmHd9htM5OAGB7-tTFf7jgzMI1jnYLc"-->_
                <!--"9hTczu-fkDkzcT9pdtsf0mVFViOxhsg27F08pHvlprA"-->_
                <!--"hcTmLh_Cojhg5bcfJ6dO6SnSw0Z-aNG6pVtxpGhGkK0"-->_
                <table class="tb list rw-table">_
                  <caption>"Anmeldefristen"</caption>_
                  <tbody>
                    <tr>_
                      <td class="tbsubhead">" Phase "</td>_
                      <td class="tbsubhead">" Block "</td>_
                      <td class="tbsubhead">" Start "</td>_
                      <td class="tbsubhead">" Ende Anmeldung "</td>_
                      <td class="tbsubhead">" Ende Abmeldung"</td>_
                      <td class="tbsubhead">" Ende Hörer "</td>_</tr>_
                    <tr>_
                      <td class="tbdata">" Direkte Zulassung "</td>_
                      <td class="tbdata">" Vorlesungszeit "</td>_
                      <td class="tbdata">" 01.09.2024 00:00 "</td>_
                      <td class="tbdata">" 28.02.2025 23:59 "</td>_
                      <td class="tbdata">" 28.02.2025 23:59 "</td>_
                      <td class="tbdata">" 28.02.2025 23:59 "</td>_</tr>_</tbody></table>_
                <!--"jqi9g3rkaAfzvYMoNoUy1kaNO-LZHLBDXL8OW4hAioM"-->_
                <!--"y8Y0kF-8a-W4aY1VMRgIGgsP_KmWzGK6jhpfDWop4Wc"-->_
                <table class="tb list rw-table rw-all">_
                  <caption>"Termine"</caption>_
                  <tbody>
                    <tr class="rw-hide">_
                      <td class="tbsubhead"></td>_
                      <td class="tbsubhead" style="width:120px;">"Datum"</td>_
                      <td class="tbsubhead">"Von"</td>_
                      <td class="tbsubhead">"Bis"</td>_
                      <td class="tbsubhead">"Raum"</td>_
                      <td class="tbsubhead">"Lehrende"</td>_</tr>_

    };
    while html_handler.peek().is_some() {
        html_handler = {
            html_extractor::html! {
                           <tr>_
                              <td class="tbdata rw">id</td>_
                              <td class="tbdata rw rw-course-date" name="appointmentDate">date</td>_
                              <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">time_start</td>_
                              <td class="tbdata rw rw-course-to" name="appointmentDateTo">time_end</td>_
                              <td class="tbdata rw rw-course-room">_
                                <a name="appointmentRooms" href=room_url>room</a>_</td>_
                              <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">instructors
                          </td>_</tr>_
            }
            html_handler
        }
    }
    html_extractor::html! {
                      </tbody></table>_
                <!--" Termine END "-->_
                <!--" PreCoRequisites Start"-->_
                <!--" PreCoRequisites End"-->_
                <!--" Cluster START "-->_
                <!--" Cluster END "-->_
                <!--" Contained in modules START "-->_
                <table class="tb rw-table rw-all">_
                  <caption>"Enthalten in Modulen"</caption>_
                  <tbody>
                    <tr>_
                      <td class="tbsubhead">"Modul"</td>_</tr>_
                    <tr>_
                      <td class="tbdata">"20-00-0040 Graphische Datenverarbeitung I (WiSe 2021/22) "</td>_</tr>_
                    <tr>_
                      <td class="tbdata">"20-00-0040 Graphische Datenverarbeitung I (WiSe 2022/23) "</td>_</tr>_
                    <tr>_
                      <td class="tbdata">"20-00-0040 Graphische Datenverarbeitung I (WiSe 2023/24) "</td>_</tr>_
                    <tr>_
                      <td class="tbdata">"20-00-0040 Graphische Datenverarbeitung I (WiSe 2024/25) "</td>_</tr>_</tbody></table>_
                <!--" Contained in modules END "-->_
                <!--" exams within modules START"-->_
                <!--" Exams within Modules END"-->_
                <!--" Exams Start "-->_
                <!--" Exams End "-->_
                <!--" Show course catalogues "-->_
                <!--" End New Part "-->_
                <!--" End of Main Content "-->_</div>_
              <!--" Side Content "-->_
              <!--" Navigator start "-->_
              <div class="contentlayoutright" id="contentlayoutright">_
                <div class="tb courseList">_
                  <div class="tbhead">"Übersicht der Kurstermine"</div>_
                  <ul class="courseList">_
                    <li class="courseListCell numout" title="Mo, 14. Okt. 2024 / 09:50:00 - 11:30:00 / S103/123">"1"</li>_
                    <li class="courseListCell numout" title="Mo, 21. Okt. 2024 / 09:50:00 - 11:30:00 / S103/123">"2"</li>_
                    <li class="courseListCell numout" title="Mo, 28. Okt. 2024 / 09:50:00 - 11:30:00 / S103/123">"3"</li>_
                    <li class="courseListCell numout" title="Mo, 4. Nov. 2024 / 09:50:00 - 11:30:00 / S103/123">"4"</li>_
                    <li class="courseListCell numout" title="Mo, 11. Nov. 2024 / 09:50:00 - 11:30:00 / S103/123">"5"</li>_
                    <!--" </tr></tr> NEW TABLEROW NOT NEEDED IN LIST"-->_
                    <li class="courseListCell numout" title="Mo, 18. Nov. 2024 / 09:50:00 - 11:30:00 / S103/123">"6"</li>_
                    <li class="courseListCell numout" title="Mo, 25. Nov. 2024 / 09:50:00 - 11:30:00 / S103/123">"7"</li>_
                    <li class="courseListCell numout" title="Mo, 2. Dez. 2024 / 09:50:00 - 11:30:00 / S103/123">"8"</li>_
                    <li class="courseListCell numout" title="Mo, 9. Dez. 2024 / 09:50:00 - 11:30:00 / S103/123">"9"</li>_
                    <li class="courseListCell numout" title="Mo, 16. Dez. 2024 / 09:50:00 - 11:30:00 / S103/123">"10"</li>_
                    <!--" </tr></tr> NEW TABLEROW NOT NEEDED IN LIST"-->_
                    <li class="courseListCell numout" title="Mo, 13. Jan. 2025 / 09:50:00 - 11:30:00 / S103/123">"11"</li>_
                    <li class="courseListCell numout" title="Mo, 20. Jan. 2025 / 09:50:00 - 11:30:00 / S103/123">"12"</li>_
                    <li class="courseListCell numout" title="Mo, 27. Jan. 2025 / 09:50:00 - 11:30:00 / S103/123">"13"</li>_
                    <li class="courseListCell numout" title="Mo, 3. Feb. 2025 / 09:50:00 - 11:30:00 / S103/123">"14"</li>_
                    <li class="courseListCell numout" title="Mo, 10. Feb. 2025 / 09:50:00 - 11:30:00 / S103/123">"15"</li>_
                    <!--" </tr></tr> NEW TABLEROW NOT NEEDED IN LIST"-->_</ul>_</div>_
                <table class="tb rw-table">_
                  <tbody>
                    <tr class="rw-all">_
                      <td class="tbhead">"Lehrende"</td>_</tr>_
                    <tr>_
                      <td class="tbdata" name="instructorTitle">"\n                                                Prof. Dr. techn. Wolf Dietrich Fellner\n                                        "</td>_</tr>_</tbody></table>_
                <!--" Navigator end "-->_</div>_</form>_
            <script type="text/javascript">"\n<!--\nvar Cal=document.getElementById(\"calendar\");\nif(Cal){\n        Cal.style.display=\"inline\";\n}\n\n//-->\n"</script>_
            <noscript></noscript>_
            <!--"
                        © DATENLOTSEN INFORMATIONSSYSTEME GMBH
                        e-mail:                 info@datenlotsen.de
                        web:                    http://www.datenlotsen.de

                        customer:               tuda
                        version:                13.00.016
                        filename:               foot.htm
                //"-->_</div>_</div>_</div>_
      <div id="pageFoot" class="pageElementTop">_
        <div id="pageFootControls" class="pageElementTop">_
          <div id="pageFootControlsLeft">_
            <a href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N952337203336396,-N000311,-Aimprint" class="img img_arrowImprint pageElementLeft" id="pageFootControl_imp">"Impressum"</a>_
            <a href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N952337203336396,-N000311,-Acontact" class="img img_arrowContact pageElementLeft" id="pageFootControl_con">"Kontakt"</a>_
            <a href="#" onclick="window.print();" class="img img_arrowPrint pageElementLeft" id="pageFootControl_pri">"Drucken"</a>_</div>_
          <div id="pageFootControlsRight">_
            <a href="#top" class="img img_arrowUp pageElementRight" id="pageFootControl_up">_</a>_</div>_</div>_</div>_</div>_
    <div id="IEdiv">_</div>
    <!--" purpose of this is to avoid a pretty strange bug in IE's "-->_
    <!--" in case the schedule exceeds the given width in lower IE's"-->_
    <!--""-->_
    <div class="invAnchor">_
      <a name="bottom" class="invAnchor">_</a>_</div>_</body></html>
    }
    Ok(CourseDetailsResponse {})
}
