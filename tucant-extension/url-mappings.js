
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
]