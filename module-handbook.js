import { getDocument } from "pdfjs-dist/legacy/build/pdf.mjs";

// node module-handbook.js

const document = await getDocument("/home/moritz/Downloads/2023_05_11_MHB_MSC_INF.pdf").promise
console.log(document)