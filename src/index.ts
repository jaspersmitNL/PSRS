import { Lexer } from "./lexer";
import { Parser } from "./parser";

const source = await Bun.file("test.ps1").text();
const lexer = new Lexer(source);
const tokens = lexer.tokenize();

const parser = new Parser(tokens);
const result = parser.value();

console.log(result);

Bun.write("test.json", JSON.stringify(result, null, 2));
