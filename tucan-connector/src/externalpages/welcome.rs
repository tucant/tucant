use scraper::Html;

use crate::{
    common::head::{footer, html_head, logged_out_head},
    html_handler::Root,
    MyClient, TucanError,
};

pub async fn welcome(client: &MyClient) -> Result<(), TucanError> {
    let response = client.get("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome")
    .send()
    .await?
    .error_for_status()?;
    let content = response.text().await?;
    let document = Html::parse_document(&content);
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
        <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de" xmlns:msdt="uuid:C2F41010-65B3-11d1-A29F-00AA00C14882" xmlns:mso="urn:schemas-microsoft-com:office:office">
            <head>_
    };
    let html_handler = html_head(html_handler);
    html_extractor::html! {
            <style type="text/css">
                "oiK6m4ZNKQoGD_x_6V3-YFNSsLMUaXrX5lQwN4Q88fc"
            </style>_
        </head>_
        <body class="external_pages">_
    };
    let html_handler = logged_out_head(html_handler, 344);
    html_extractor::html! {
                    <!--"Ur30ahmaXh5XzV5xIHsTj20h-0qX1_GS1SR0QttvqB0"-->_
                    <script type="text/javascript">
                    </script>_
                    <!--"1SdyF9DDr8Z_kEcqcOdFHDujurFGmYcPovwfandPimw"-->_
                    <meta http-equiv="content-type" content="text/html; charset=windows-1252"></meta>
                    <div id="inhalt" style="padding:0px; width:650px; margin:0px; background-color:#ffffff;">_
                        <h1>
                            "Herzlich willkommen bei TUCaN, dem Campus-Management-System der "
                            <br></br>
                            "TU Darmstadt! "
                        </h1>_
                        <!--"rjV7X6SdGjjerKiAcwXSu6am9MFlzsqzZJpMF0QGvyc"-->_
                        <!--"QZYtNUT0elp2c-JwCE6e-d0tQPEo53cyPn2Gq13180w"-->_
                        <br></br>_
                        <!--"Ha9yU5aVvqveCwalKN4D9fNhg1O3MnuK8ck8kat0mAo"-->_
                        <p style="line-height: 140%;">
                            <strong>
                                "Studierende, Lehrende, Stellvertretungen und Mitarbeitende der TU Darmstadt"
                            </strong>
                            <br></br>
                            "\nmelden sich mit ihrer TU-ID an, um das System zu nutzen."
                        </p>_
                        <ul>_
                            <li>
                                <a href="https://www.tu-darmstadt.de/studieren/studierende_tu/studienorganisation_und_tucan/index.de.jsp" target="_blank">
                                    "FAQ für Studierende"
                                </a>
                            </li>_
                            <li>
                                <a href="https://www.intern.tu-darmstadt.de/dez_ii/campusmanagement/cm_tucan/infos_fuer_lehrende/index.de.jsp" target="_blank">
                                    "FAQ für Lehrende"
                                </a>
                            </li>
                        </ul>_
                        <p style="line-height: 40%;">_
                        </p>_
                        <p style="line-height: 140%;">
                            <strong>
                                "Bewerber:innen und Gasthörer:innen"
                            </strong>
                            <br></br>
                            "\nlegen sich zunächst ein "
                            <a href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000410,-Atucan%5Faccount%2Ehtml">
                                "TUCaN-Account"
                            </a>
                            "\n an,\n um ihre Zugangsdaten zu erhalten und melden sich anschließend mit \ndiesen Zugangsdaten an, bis sie ihre endgültige TU-ID erhalten."
                        </p>_
                        <ul>_
                            <li>
                                <a href="https://www.tu-darmstadt.de/studieren/studieninteressierte/bewerbung_zulassung_tu/online_bewerbung/index.de.jsp" target="_blank">
                                    "FAQ für Bewerber:innen"
                                </a>
                            </li>_
                            <li>
                                <a href="https://www.tu-darmstadt.de/gasthoerer" target="_blank">
                                    "FAQ für Gasthörer:innen"
                                </a>
                            </li>_
                        </ul>_
                        <p style="line-height: 40%;">_
                        </p>_
                        <p style="line-height: 140%;">
                            <strong>
                                "Promovierende zur Registrierung / Einschreibung"
                            </strong>
                            <br></br>
                            "\nbeachten bitte die Informationen auf den "
                            <a href="http://www.tu-darmstadt.de/promotion-registrierung" target="_blank">
                                "Webseiten"
                            </a>
                            ". "
                        </p>_
                        <p style="line-height: 40%;">_
                        </p>_
                        <!--"DdY7X0SUBoVh1HeLdKUt8ZGyIAO6W4ecYeXtgEC_uu8"-->_
                        <!--"8BGIyQ2B-rACsM51dW_-fXQOxEtSSMQKmvACrZeN8RM"-->_
                        <!--"kxxdx9oC13X6nNfsroMEL83B9YcEzTaGRyJ7fJawlxs"-->_
                        <!--"W2wZ2lO1RgblnmkfAkpqqZROzL9YntinBONQb0VR21U"-->_
                        <div style="padding:10px; width:650px; border:thin solid grey; margin:0px; background-color:#f8f9ed;">_
                            <p style="line-height: 140%;">_
                                <strong>
                                    "Aktuelles: Fristen zur Prüfungsanmeldung in TUCaN für das Wintersemester 2024/2025\n\n"
                                </strong>_
                            </p>_
                            <p style="line-height: 140%;">
                                "Die Anmeldezeit zu Prüfungen im WiSe 2024/2025 hat in der Regel am 15. November 2024 begonnen."
                                <br></br>
                                " In vielen Studiengängen endet die Anmeldefrist am 15. Dezember 2024 - bitte informieren Sie sich rechtzeitig! Ihre Anmeldung nehmen Sie im TUCaN-Webportal im Bereich "
                                <i>
                                    "Prüfungen"
                                </i>
                                " unter "
                                <i>
                                    "Meine Prüfungen / Anmeldung zu Prüfungen"
                                </i>
                                " vor."
                            </p>_
                            <p style="line-height: 140%;">
                                "Fachbereiche können darüber hinaus individuelle Fristen festlegen. Die An- und Abmeldefristen entnehmen Sie bitte den "
                                <a href="http://www.tu-darmstadt.de/tucan-pruefungsdetails" target="_blank">
                                    "Prüfungsdetails"
                                </a>
                                " in TUCaN."
                            </p>
                            "\n\n\n\n\n→ "
                            <a href="https://www.tu-darmstadt.de/studieren/studierende_tu/studienorganisation_und_tucan/hilfe_und_faq/index.de.jsp" target="_blank">
                                "Hilfe & FAQ zur Prüfungsanmeldung"
                            </a>
                            <br></br>_
                            <p>
                            </p>_
                            <br></br>_
                        </div>_
                        <!--"jGv5521IKCGoJrYXj5NxPjxZEq5zcNxnkwtMxKZLIX0"-->_
                        <p>_
                        </p>
                        "\n→ "
                        <a href="https://www.tu-darmstadt.de/studieren/studierende_tu/studienorganisation_und_tucan/hilfe_und_faq/artikel_details_de_en_37312.de.jsp" target="_blank">
                            "TUCaN Wartungszeit: Dienstag um 6 - 9 Uhr"
                        </a>_
                        <br></br>
                        <br></br>
                        "\n→ "
                        <a href="https://www.tu-darmstadt.de/studieren/studierende_tu/studienorganisation_und_tucan/hilfe_und_faq/artikel_details_de_en_344192.de.jsp" target="_blank">
                            "Hinweise zum Datenschutz"
                        </a>_
                        <!--"Diq-FIUkmF-JjcTgujrkufLubS6eenSQeBajtbBaVPw"-->_
                        <p>_
                        </p>_
                        <!--"IecUhiUBkSqz3ZJqC7gry_m5yl8ydiVd5GKzGwpO-ns"-->_
                        <title>
                        </title>
                    </div>_
                </div>_
            </div>_
        </div>_
    };
    let html_handler = footer(html_handler, 1, 344);
    html_extractor::html! {
        <!--"kPihWIZIb5OjP2_N9Uh_xbuyYvDKOGNxX5S2d7yQjKY"-->
        <!--"fS28-ufck45gusNkaJA-yHsPF7qDLp0dqCxzpxz56og"-->
    }
    html_handler.end_document();
    Ok(())
}
