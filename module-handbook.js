import { getDocument, OPS } from "pdfjs-dist/legacy/build/pdf.mjs";

const OPS_INVERTED = Object.fromEntries(
    Object.entries(OPS).map(([key, value]) => [value, key])
);

// node module-handbook.js

const document = await getDocument("/home/moritz/Downloads/2023_05_11_MHB_MSC_INF.pdf").promise
//console.log(document)

// https://github.com/mozilla/pdf.js/blob/master/examples/text-only/pdf2svg.mjs

for (let i = 1; i <= document.numPages; i++) {
    const page = await document.getPage(i)
    const opList = await page.getOperatorList();

    // Walk through operator list
    for (let i = 0; i < opList.fnArray.length; i++) {
        const fnId = opList.fnArray[i];
        const args = opList.argsArray[i];

        const opName = OPS_INVERTED[fnId];
        console.log(`Operation: ${opName}`, args);
    }
}