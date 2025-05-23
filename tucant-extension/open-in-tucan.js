const EXT_PAGE_INDEX_HTML = chrome.runtime.getURL('/dist/index.html');

export async function getCurrentTab() {
    let queryOptions = { active: true, lastFocusedWindow: true };
    // `tab` will either be a `tabs.Tab` instance or `undefined`.
    // TODO FIXME typescript is wrong here
    let [tab] = await chrome.tabs.query(queryOptions);
    return tab;
}

/**
 * @param {string | undefined} id
 * @param {number} tabId
 * @param {string} url
 * @returns {string|undefined}
 */
export function handleOpenInTucan(id, tabId, url) {
    let match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N\\d+,-N\\d+,(.*)$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/module-details/${match[1]}`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N\\d+,-N\\d+,(.*)$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/course-details/${match[1]}`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N\\d+,-N\\d+,(.*)$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/registration/${match[1]}`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N\\d+,-N\\d+,$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/overview`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=(.*)$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/vv/${match[1]}`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MYMODULES&ARGUMENTS=-N\\d+,-N\\d+,$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/my-modules`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=PROFCOURSES&ARGUMENTS=-N\\d+,-N\\d+,$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/my-courses`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MYEXAMS&ARGUMENTS=-N\\d+,-N\\d+,$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/my-exams`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=EXAMRESULTS&ARGUMENTS=-N\\d+,-N\\d+,$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/exam-results`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=COURSERESULTS&ARGUMENTS=-N\\d+,-N\\d+,$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/course-results`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N\\d+,-N\\d+,$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/my-documents`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N\\d+,-N000316,-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/student-result/default`
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N\\d+,-N000316,-N0,-N000000000000000,-N000000000000000,-N(\\d+),-N0,-N000000000000000$", "g").exec(url)
    if (match) {
        return `${EXT_PAGE_INDEX_HTML}#/student-result/${match[1]}`
    }

    // ------------------------------

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/course-details/(.*)$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N${id},-N000274,${match[1]}`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/module-details/(.*)$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N${id},-N000275,${match[1]}`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/registration/(.*)$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N${id},-N000311,${match[1]}`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N${id},-N000019,`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/overview$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N${id},-N000019,`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/my-modules$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYMODULES&ARGUMENTS=-N${id},-N000275,`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/my-courses$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PROFCOURSES&ARGUMENTS=-N${id},-N000274,`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/my-exams$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYEXAMS&ARGUMENTS=-N${id},-N000318,`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/exam-results$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXAMRESULTS&ARGUMENTS=-N${id},-N000325,`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/course-results$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSERESULTS&ARGUMENTS=-N${id},-N000324,`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/my-documents$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N${id},-N000557,`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/student-result/default$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N${id},-N000316,-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000`
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/student-result/(.*)$`, "g").exec(url)
    if (id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N${id},-N000316,-N0,-N000000000000000,-N000000000000000,-N${match[1]},-N0,-N000000000000000`
    }

    // --------------------------------

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/$`, "g").exec(url)
    if (!id && match) {
        return `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome`
    }

    if (!id) {
        chrome.notifications.create({
            type: "basic",
            iconUrl: chrome.runtime.getURL("/icon-512.png"),
            title: "Not logged in",
            message: "Could not detect session, please login again",
        });
        return undefined;
    }

    chrome.notifications.create({
        type: "basic",
        iconUrl: chrome.runtime.getURL("/icon-512.png"),
        title: "URL not supported",
        message: "Unfortunately this URL is not supported yet. We welcome any contribution",
    });
    return undefined;
}
