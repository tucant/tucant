use html_extractor::html;

use crate::html_handler::{AfterDoctype, InElement, InRoot, Root};

pub fn html_head<'a>(
    html_handler: InElement<'a, InElement<'a, InRoot<'a, Root<'a>, AfterDoctype>>>,
) -> InElement<'a, InElement<'a, InRoot<'a, Root<'a>, AfterDoctype>>> {
    html! {
        <title>"Technische Universität Darmstadt"</title>_
            <!--"iPRTdQsauRZVOSpz0PmEl_ubhHccJRCaNU_bI6seaq0"-->_
            <!--"muh4fptckC_Ch7T74xLI7ivPp07sWskCVg2gy3woY28"-->_
            <meta http-equiv="X-UA-Compatible" content="IE=edge"></meta>_
            <meta http-equiv="cache-control" 		content="no-cache"></meta>_
            <meta http-equiv="expires" 				content="-1"></meta>_
            <meta http-equiv="pragma" 				content="no-cache"></meta>_
            <meta http-equiv="Content-Type" 		content="text/html; charset=utf-8"></meta>_
            <meta http-equiv="Content-Script-Type"	content="text/javascript"></meta>_
            <meta name="referrer" content="origin"></meta>_
            <meta name="keywords" content="Datenlotsen,Datenlotsen Informationssysteme GmbH,CampusNet,Campus Management"></meta>_
            <!--"PVD_IUFslfLcokMkhhqUJ2XUD8f4-KrQiSrt7qeobqU"-->_
            <link rel="shortcut icon" type="image/x-icon" href="/gfx/tuda/icons/favicon.ico"></link>_
            <script src="/js/jquery-3.6.0.min.js" 	type="text/javascript"></script>_
            <script src="/js/checkDate.js" 	type="text/javascript"></script>_
            <script src="/js/edittext.js" 	type="text/javascript"></script>_
            <script src="/js/skripts.js" 	type="text/javascript"></script>_
            <script src="/js/x.js" 			type="text/javascript"></script>_
            <!-- "-cBtAUCsH5L1QCSAXhrWUyjqREZ-qAM6anBuGb0jpis"-->_
            <link id="defLayout" 	href="/css/_default/def_layout.css"	rel="stylesheet"  type="text/css"	media="screen"></link>_
            <link id="defMenu" 		href="/css/_default/def_menu.css" 	rel="stylesheet"  type="text/css"	media="screen"></link>_
            <link id="defStyles" 	href="/css/_default/def_styles.css"	rel="stylesheet"  type="text/css"	></link><!-- "8ohjoL_DKlESc2buIQIqpMDCPv88imAStNDyjxAd2yY" -->_
            <link id="pagePrint" 	href="/css/_default/def_print.css" 	rel="stylesheet"  type="text/css"	media="print"></link>_
            <!-- "tsCXIkgf7AHAT6f4SFdkYqr9qZ1RI2wPidDGXYoyb-M" -->_
            <link id="pageStyle"		href="/css/styles.css"	rel="stylesheet"	type="text/css"  	></link>_<!-- "8ohjoL_DKlESc2buIQIqpMDCPv88imAStNDyjxAd2yY" -->_
            <link id="pageColors"		href="/css/colors.css"	rel="stylesheet" 	type="text/css" 	media="screen" ></link>_
            <!--"Bv96RRDpRJh6Mov4faCHyudSmfHE7HfK_7sTjNxd1wY" -->_
            <!--"dIBUikqFO2tcT78tvc7dv_E180BxF6LhwTNb4gpSuQM"-->_
            <!--"fD1xdYETGI2QrMhnwhN-3obm-UIuRhNpzKv2Qbz53Ac"-->_
            <!--"NIHfntnP_QYxOqBt0vrT3UIfpe7DzzHCCiQbHrVLrXE"-->_
            <!--"x2WUiOGjWA_UDiUqZA9skrh_uNAWGlcC-R__ip9vYyg"-->_
    }
    html_handler
}
