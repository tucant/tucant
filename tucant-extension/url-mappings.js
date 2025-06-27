/**
 * @typedef MappingType
 * @type {object}
 * @property {string} from
 * @property {string|null} to null means interpolate here
 */

/**
 * 
 * @param {TemplateStringsArray} strings 
 * @param {MappingType[]} args
 * @returns {{ strings: TemplateStringsArray; args: MappingType[]; }} 
 */
function t(strings, ...args) {
    return {
        strings,
        args
    }
}

/**
 * 
 * @param {number} number
 * @returns {MappingType} 
 */
function num(number) {
    return {
        from: "\\d+",
        to: number.toString()
    }
}

/**
 * 
 * @param {string} string
 * @returns {MappingType} 
 */
function s(string) {
    return {
        from: string,
        to: null // null means interpolate here
    }
}

export const bidirectionalMappings = (/** @type {number} */ id) => [
    {
        "tucan": t`PRGNAME=MODULEDETAILS&ARGUMENTS=-N${num(id)},-N${num(275)},${s("(.*)")}`,
        "tucant": t`module-details/${s("(.*)")}`,
    },
    {
        "tucan": t`PRGNAME=COURSEDETAILS&ARGUMENTS=-N${num(id)},-N${num(274)},${s("(.*)")}`,
        "tucant": t`course-details/${s("(.*)")}`,
    },
    {
        "tucan": t`PRGNAME=REGISTRATION&ARGUMENTS=-N${num(id)},-N${num(311)},${s("(.*)")}`,
        "tucant": t`registration/${s("(.*)")}`,
    },
    {
        "tucan": t`PRGNAME=MLSSTART&ARGUMENTS=-N${num(id)},-N${num(19)},`,
        "tucant": t`overview`,
    },
    {
        "tucan": t`PRGNAME=ACTION&ARGUMENTS=${s("(.*)")}`,
        "tucant": t`vv/${s("(.*)")}`,
    },
    {
        "tucan": t`PRGNAME=MYMODULES&ARGUMENTS=-N${num(id)},-N${num(275)},`,
        // TODO FIXME my-modules/current this is actually not current but recent as it remembers last navigation
        "tucant": t`my-modules/current`,
    },
    {
        "tucan": t`PRGNAME=MYMODULES&ARGUMENTS=-N${num(id)},-N${num(275)},-N999`,
        "tucant": t`my-modules/all`,
    },
    {
        "tucan": t`PRGNAME=MYMODULES&ARGUMENTS=-N${num(id)},-N${num(275)},-N${s("(\\d+)")}`,
        "tucant": t`my-modules/${s("(\\d+)")}`,
    },
    {
        "tucan": t`PRGNAME=PROFCOURSES&ARGUMENTS=-N${num(id)},-N${num(274)},`,
        "tucant": t`my-courses/current`,
    },
    {
        "tucan": t`PRGNAME=PROFCOURSES&ARGUMENTS=-N${num(id)},-N${num(274)},-N999`,
        "tucant": t`my-courses/all`,
    },
    {
        "tucan": t`PRGNAME=PROFCOURSES&ARGUMENTS=-N${num(id)},-N${num(274)},-N${s("(\\d+)")}`,
        "tucant": t`my-courses/${s("(\\d+)")}`,
    },
    {
        "tucan": t`PRGNAME=MYEXAMS&ARGUMENTS=-N${num(id)},-N${num(318)},`,
        "tucant": t`my-exams/current`,
    },
    {
        "tucan": t`PRGNAME=MYEXAMS&ARGUMENTS=-N${num(id)},-N${num(318)},-N999`,
        "tucant": t`my-exams/all`,
    },
    {
        "tucan": t`PRGNAME=MYEXAMS&ARGUMENTS=-N${num(id)},-N${num(318)},-N${s("(\\d+)")}`,
        "tucant": t`my-exams/${s("(\\d+)")}`,
    },
    {
        "tucan": t`PRGNAME=EXAMRESULTS&ARGUMENTS=-N${num(id)},-N${num(325)},`,
        "tucant": t`exam-results/current`,
    },
    {
        "tucan": t`PRGNAME=EXAMRESULTS&ARGUMENTS=-N${num(id)},-N${num(325)},-N999`,
        "tucant": t`exam-results/all`,
    },
    {
        "tucan": t`PRGNAME=EXAMRESULTS&ARGUMENTS=-N${num(id)},-N${num(325)},-N${s("(\\d+)")}`,
        "tucant": t`exam-results/${s("(\\d+)")}`,
    },
    {
        "tucan": t`PRGNAME=COURSERESULTS&ARGUMENTS=-N${num(id)},-N${num(324)},`,
        "tucant": t`course-results/current`,
    },
    {
        "tucan": t`PRGNAME=COURSERESULTS&ARGUMENTS=-N${num(id)},-N${num(324)},-N${s("(\\d+)")}`,
        "tucant": t`course-results/${s("(\\d+)")}`,
    },
    {
        "tucan": t`PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N${num(id)},-N${num(557)},`,
        "tucant": t`my-documents`,
    },
    {
        "tucan": t`PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome`,
        "tucant": t``,
    },
    {
        "tucan": t`PRGNAME=STUDENT_RESULT&ARGUMENTS=-N${num(id)},-N${num(316)},-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000`,
        "tucant": t`student-result/default`,
    },
    {
        "tucan": t`PRGNAME=STUDENT_RESULT&ARGUMENTS=-N${num(id)},-N${num(316)},-N0,-N000000000000000,-N000000000000000,-N${s("(\\d+)")},-N0,-N000000000000000`,
        "tucant": t`student-result/${s("(\\d+)")}`,
    },
]