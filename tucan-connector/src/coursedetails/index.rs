use scraper::Html;
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
        <script type="text/javascript">"\n<!--\n\nfunction focus (){\n        Message.focus();\n}\n\nvar Message;\nfunction popUp(datei) {\n        Message = window.open(datei,\"Message\",\"width=650,height=650,dependent=yes,scrollbars=no\");\n}\n\n// -->\n"</script>_
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
                        <!--" Reggroup,prioscheme,alloscheme,waitlist "-->_
                        <p>
                          <b>"Digitale Lehre"</b>":"
                          <br></br>"\n                                                                        \n                                Die Veranstaltung wird in Praesenz \nstattfinden, Details und etwaige Aenderungen werden in unserem Moodle \nKurs auf dem Informatik-Moodle bekannt gegeben.\n                                                \t\t\t\t\t\t\t\t\t\t\t\t"</p>_
                        <p>
                          <b>"Lehrinhalte"</b>":"
                          <br></br>"\n                                                                        \n                                Einführung in die Grundlagen der \nComputergraphik, insb. Ein- u. Ausgabegeräte, Rendering Pipeline am \nBeispiel von OpenGL, räumliche Datenstrukturen, Beleuchtungsmodelle, Ray\n Tracing, aktuelle Entwicklungen in der Computergraphik\n"
                          <br></br>_
                          <br></br>
                          <b>"Qualifikationsziele / Lernergebnisse:"</b>_
                          <br></br>"Nach erfolgreichem Besuch dieser Veranstaltung sind Studierende in \nder Lage alle Komponenten der Graphikpipeline zu verstehen und dadurch \nvariable Bestandteile (Vertex-Shader, Fragment-Shader, etc.) anzupassen.\n Sie können Objekte im 3D-Raum anordnen, verändern und effektiv \nspeichern, sowie die Kamera und die Perspektive entsprechend wählen und \nverschiedene Shading-Techniken und Beleuchtungsmodelle nutzen, um alle \nSchritte auf dem Weg zum dargestellten 2D-Bild anzupassen.\n                                                \t\t\t\t\t\t\t\t\t\t\t\t"</p>_
                        <p>
                          <b>"Literatur"</b>":"
                          <br></br>"\n                                                                        \n                                - Real-Time Rendering: Tomas \nAkenine-Möller, Eric Haines, Naty Hoffman A.K. Peters Ltd., 3rd edition,\n ISBN 987-1-56881-424-7\n"
                          <br></br>"- Fundamentals of Computer Graphics: Peter Shirley, Steve Marschner, third edition, ISBN 979-1-56881-469-8\n"
                          <br></br>"- Weitere aktuelle Literaturhinweise werden in der Veranstaltung gegeben.\n                                                \t\t\t\t\t\t\t\t\t\t\t\t"</p>_
                        <p>
                          <b>"Voraussetzungen"</b>":"
                          <br></br>"\n                                                                                                        - Programmierkenntnisse\n"
                          <br></br>"- Grundlegende Algorithmen und Datenstrukturen\n"
                          <br></br>"- Lineare Algebra\n"
                          <br></br>"- Analysis\n"
                          <br></br>"- Inhalte der Vorlesung Visual Computing\n"
                          <br></br>_</p>_
                        <p>
                          <b>"Weitere Informationen"</b>":"
                          <br></br>"\n                                                                        \n                                 IV, 6 CP/4SWS, i.d.R. jedes \nWintersemester\n                                                \t\t\t\t\t\t\t\t\t\t\t\t"</p>_
                        <p>
                          <b>"Online-Angebote"</b>":"
                          <br></br>"\n                                                                        \n                                Kurs im Moodle des FB 20 \n(\"Informatiker-Moodle\", https://moodle.informatik.tu-darmstadt.de/)\n                                                \t\t\t\t\t\t\t\t\t\t\t\t"</p>_
                        <!--" / LIST OF COURSE PROPS "-->_</td>_</tr>_</tbody></table>_
                <!--" KG START "-->_
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
                        <a href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N952337203336396,-N000311,-N391343674191079,-N389955196599934,-N389955196692982,-N0,-N000000000000000,-N3" class="img img_arrowLeft pageElementRight">"\n\t\t\t\t\t\t\t\t\tKleingruppe anzeigen\n\t\t\t\t\t\t\t\t"</a>_</div>_</li>_</ul>_</div>_
                <!--" KG END "-->_
                <!--" Media START "-->_
                <table class="tb rw-table">_
                  <caption>"\n                        Literatur\n                                        "</caption>_
                  <tbody>
                    <tr>_
                      <td class="tbsubhead">_
                        <span name="literatureCategory">
                          <!--"$MG_MEDIACATNAME"--></span>_</td>_</tr>_</tbody></table>_
                <!--" Media END "-->_
                <!--" Prep Start "-->_
                <!--"Prep End"-->_
                <!--" Anmeldefristen START "-->_
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
                <!--" Anmeldefristen ENDE "-->_
                <!--" Termine START "-->_
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
                    <tr>_
                      <td class="tbdata rw">"1"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 14. Okt. 2024"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A6NlWx~OfTnT9ghRtJAwb2aJ9mOW0NEjvrP6zU8mnczT4ZtMq5~UWRfxSghbHQOPjkhuoMpBenPt6ZLrg02TMcCSR4Urr6aNQjVNSPxNCdYtVh3dhdogefN~QlEBJ~MjjfuxO6vBuGuyoOA8xWyvJs7nbIim~P-LjTibj7NFgOy6U3xc_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"2"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 21. Okt. 2024"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A~2JPBeI--X0v0OlEoQTlPSxU8VgrrrlvltHfYGmSoUHbM8PMzCj2LMbKPmM9c-G3rtZcTnMGAxl6vuBCCXUQUjr-GkHc9uQg-wTBNRuUarE36XZxIhJPFdLshZgRZAjikH7hbesnc6ROlu9I0hqEkROsPlYzXfpWoQlSmZIytkC7sBE_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"3"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 28. Okt. 2024"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A1tVmBU0eEnyrdgq2xNy3MSitZkqqHHfWqd38Mu5odgDMmQIT2-NNIBOZrFtN3FvoC4z96xW8TY0SwrWfCksviWIin9NibSEq5jXcvGPflCg6UOG4Sek1xYsYzbZV75qgIsmitmALf1MjkVc5tdxxJMLKMB0at21RxZaXcNDLmmee6PA_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"4"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 4. Nov. 2024"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-Ak1ehX6~KfShdtlOORLjA7R8vJCCQnomC8P1sjxcHGwGZE9-FEcq99NtCA0B3q9dqNPY1eoOE0bko7w-D-Cye-PJQaVS~kJgJyblem8sHqBVRnjvx7V8guo7JTIhz0hXEwXQxpV4f4TupQvbH7t2FHeRwtFK-QJdsat-NbG~exwnGvkk_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"5"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 11. Nov. 2024"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A-QSw4XhwsRmGyquoYQAbKNrcbtt5r~EU6qkI7OBBtTngNfbWq7EpofhbtCFJJg2g758oqAWvMWjhFTqjB7Hdl6QvAvdd5BUYi76mhahAqp9r3nP6d~iYx5petERKie9W8n4UzZ9s6U3pOp-3N-ImOdAIRtGoviSwcGIhX01v4ee3NhI_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"6"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 18. Nov. 2024"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AMIQXVf6WZMhhS03gFIfqhEbQ2FATW-kDnwXQ7is50w~usPLdlvf7PubDT8TgDLsA2gxMZDmp~uhleopzjhKjYJhPyoEX6VeKHFt0LsrYh~V4sT1lVLqhUqwyQFXm32K4OkoCp~-iwDSsQNhgjz84bZ4TBX1JHCav4GRRv-d46Nfq7pQ_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"7"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 25. Nov. 2024"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A8xgqPHH-2KA-RRoX7AlNITWaYNnYLwsh8FtSEPB~MCD~NZfnsDhozMkqGuedu4qLGGdJTKuMjOwyIdXxnTj9UE3lCKYdKK5UGFkNXa4HDDwiGCzkMuIKfxxJVSBZ4P2fd8whPJqFvdyVzcmB8lKtz5IhK6DzPIISDTGL3czEIUvFEmQ_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"8"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 2. Dez. 2024"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AD3-Ly6nYT9Y3eqjJjcobtiATSA0vNe~tjyP5UFud3ZWbkL9E0Kl8oapqBgLxegkMsKyR6DDOyiW~tkLNIYsUGFWa8p2mi-g07daaF3YXh8YxwBidlRGyUYdfxhZmXyUC4Xy6UYhp8vRf~qsozimdDaLr3lH0r3huuM3EAFWdP1NR5BI_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"9"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 9. Dez. 2024"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AsXVNl30lo0lI13RgF~ZccEgsqXfhjGXSh14eOgZXaoCEd0qwKallQlyamviLana5Jqdqs5Ij~5Kco~Kb8Iye8tv~mRdvL409sB-De-Jmr~DAz09LDZWrHu0y1bG1rpqY4MXF1dl8FSvSAW1HfaV5PPMVQLlUa4tPQ6uaQRlOAgvMB2o_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"10"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 16. Dez. 2024"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-Arm22qx0dQAmtWKuYl~FGqlcqXyG61hvTG~0aMQkmXxCJBPlP2e97O9c4Y5SAXQIS6ilcVzE31HpXD8lcC3wiBc-c4gCynxc7Gyago09Q7XdOybFADNyf0tFwv5Est5WaUbNV3bRfrzR5EjWxhCcjOOSd-ywcMV~LdMf6LVQ~C~l1ilc_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"11"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 13. Jan. 2025"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AI6r69Vj8iEe5AtlNIJGHUfdu5rY5lvi8SvqYAMSHa8IPIt8THYT5nYKjMpfbQ11ddVie~YSkjPoZe0Bl0GX81Iy-nto~91HVGvJqScXuj-DMd8eEaI0Z9O9ApiFQGt~H-FFPIYFFPL7vDIfID-gUv3F4YuxSyz1dCGq8rcpNYB6J9CM_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"12"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 20. Jan. 2025"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A8~b3t66YofL7M26tWQcQ~P2~fXBeYYQNaycTxWl7uFr0qOtXKkUnWDh1UeQoQq84WrgSbqpN7C3YjTM7SqhYdR9I0gQzFY7e5dSG6n0JdU8wwOYpmUcAlIwvl51crkiI-FiLKTLnubdWAuVxvXOxMj-JUjoSpTZ1qT2PzQ4o3kHrMvg_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"13"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 27. Jan. 2025"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AHKKcllJ2zvm~Qw4AcvTKTlOWRInP~8jBlvqVUXbcWQVZq~u~jMBDxi26c~LSXO0ncee0KoNThhDHGOMJtpuFL9v3VxK7Nm1wlGVTqb6OsQ3pL7pG85qjBbBPz1eoW1us78aJ7ZT9WhipER0KognqHaQbKChHskXb1pCnbtXSWtV~~fA_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"14"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 3. Feb. 2025"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-ATuO-3oBzGFaAejCL3fV29YLtcF0TZdW2y7M6yoWUisk6xPmckLYW0b00uoFnxygk5P8n4C~DDOWLWi0pMctyPcOukgYSzwS7uwgKtFL2zFkDonFkYFGzV8bm2pMUnTMAN2Q4NqWF4DU1seSf7xCQPAAJ8pYCBAkVXxCRAB-zmh4M3~4_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_
                    <tr>_
                      <td class="tbdata rw">"15"</td>_
                      <td class="tbdata rw rw-course-date" name="appointmentDate">"Mo, 10. Feb. 2025"</td>_
                      <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">"09:50"</td>_
                      <td class="tbdata rw rw-course-to" name="appointmentDateTo">"11:30 "</td>_
                      <td class="tbdata rw rw-course-room">_
                        <a name="appointmentRooms" href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AO~UgmglKL2UOJk4FArrAYxftZOI5gO7mctyEPbX5BM-CbWgA8uSv32VDUt~HaSt~pl8wiGBKTETcANZxgjE00soGKDvrI2wv2qmvMi2V-rAIfsEo-Jg6kvlEUWe9zKg5UguLtydzIqGY5LNgIeDIs521X99JTnIvXXLtCr9atRVe5Fs_">"S103/123"</a>_</td>_
                      <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">"\n                                                                    Prof. Dr. techn. Wolf Dietrich Fellner\n                                                    "</td>_</tr>_</tbody></table>_
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
