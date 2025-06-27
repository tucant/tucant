
/**
 * 
 * @param {*} strings 
 * @param {*} args
 * @returns {string} 
 */
function t(strings, ...args) {

}

/**
 * 
 * @param {*} number
 * @returns {string} 
 */
function num(number) {
    // \\d+
}

export const bidirectionalMappings = (/** @type {number} */ id) => [
    {
        "tucan": t`PRGNAME=MODULEDETAILS&ARGUMENTS=-N${num(id)},-N${num(275)},${"(.*)"}`,
        "tucant": t`module-details/${"(.*)"}`,
    },
    {
        "tucan": t`PRGNAME=COURSEDETAILS&ARGUMENTS=-N${num(id)},-N${num(275)},${"(.*)"}`,
        "tucant": t`course-details/${"(.*)"}`,
    },
    {
        "tucan": t`PRGNAME=REGISTRATION&ARGUMENTS=-N${num(id)},-N${num(275)},${"(.*)"}`,
        "tucant": t`registration/${"(.*)"}`,
    },
    {
        "tucan": t`PRGNAME=MLSSTART&ARGUMENTS=-N${num(id)},-N${num(275)},`,
        "tucant": t`overview`,
    },
    {
        "tucan": t`PRGNAME=ACTION&ARGUMENTS=${"(.*)"}`,
        "tucant": t`vv/${"(.*)"}`,
    },
    {
        "tucan": t`PRGNAME=MYMODULES&ARGUMENTS=-N${num(id)},-N${num(275)},`,
        "tucant": t`my-modules`,
    },
    {
        "tucan": t`PRGNAME=PROFCOURSES&ARGUMENTS=-N${num(id)},-N${num(275)},`,
        "tucant": t`my-courses`,
    },
    {
        "tucan": t`PRGNAME=MYEXAMS&ARGUMENTS=-N${num(id)},-N${num(275)},`,
        "tucant": t`my-exams/current`,
    },
    {
        "tucan": t`PRGNAME=MYEXAMS&ARGUMENTS=-N${num(id)},-N${num(275)},-N999`,
        "tucant": t`my-exams/all`,
    },
    {
        "tucan": t`PRGNAME=MYEXAMS&ARGUMENTS=-N${num(id)},-N${num(275)},-N${"(\\d+)"}`,
        "tucant": t`my-exams/${"(\\d+)"}`,
    },
    {
        "tucan": t`PRGNAME=EXAMRESULTS&ARGUMENTS=-N${num(id)},-N${num(275)},`,
        "tucant": t`exam-results/current`,
    },
    {
        "tucan": t`PRGNAME=EXAMRESULTS&ARGUMENTS=-N${num(id)},-N${num(275)},-N999`,
        "tucant": t`exam-results/all`,
    },
    {
        "tucan": t`PRGNAME=EXAMRESULTS&ARGUMENTS=-N${num(id)},-N${num(275)},-N${"(\\d+)"}`,
        "tucant": t`exam-results/${"(\\d+)"}`,
    },
    {
        "tucan": t`PRGNAME=COURSERESULTS&ARGUMENTS=-N${num(id)},-N${num(275)},`,
        "tucant": t`course-results/current`,
    },
    {
        "tucan": t`PRGNAME=COURSERESULTS&ARGUMENTS=-N${num(id)},-N${num(275)},-N${"(\\d+)"}`,
        "tucant": t`course-results/${"(\\d+)"}`,
    },
    {
        "tucan": t`PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N${num(id)},-N${num(275)},`,
        "tucant": t`my-documents`,
    },
    {
        "tucan": t`PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome`,
        "tucant": t``,
    },
    {
        "tucan": t`PRGNAME=STUDENT_RESULT&ARGUMENTS=-N${num(id)},-N000316,-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000`,
        "tucant": t`student-result/default`,
    },
    {
        "tucan": t`PRGNAME=STUDENT_RESULT&ARGUMENTS=-N${num(id)},-N000316,-N0,-N000000000000000,-N000000000000000,-N${"(\\d+)"},-N0,-N000000000000000`,
        "tucant": t`student-result/${"(\\d+)"}`,
    },
]