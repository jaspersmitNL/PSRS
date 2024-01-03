/*
    String,
    Number,
    @, {}, ()
*/
export enum TokenType {
  String = "String",
  Number = "Number",
  Identifier = "Identifier",
  Boolean = "Boolean",
  Null = "Null",
  At = "@",
  Equals = "=",
  LBrace = "{",
  RBrace = "}",
  LParen = "(",
  RParen = ")",
  EOF = "EOF",
}

export class Token {
  type: TokenType;
  value: string | number | boolean | null;
  line: number;

  constructor(type: TokenType, value: any, line: number) {
    this.type = type;
    this.value = value;
    this.line = line;
  }
}

function isAlpha(c: string): boolean {
  return (
    (c >= "a" && c <= "z") || (c >= "A" && c <= "Z") || c == "_" || c == "$"
  );
}
function isDigit(c: string): boolean {
  return c >= "0" && c <= "9";
}

function isAlphaNumeric(c: string): boolean {
  return isAlpha(c) || isDigit(c);
}

const keywords: Map<string, TokenType> = new Map([
  ["$true", TokenType.Boolean],
  ["$false", TokenType.Boolean],
  ["$null", TokenType.Null],
]);

export class Lexer {
  source: string;
  i: number;
  line: number;

  constructor(source: string) {
    this.source = source;
    this.i = 0;
    this.line = 1;
  }

  tokenize(): Token[] {
    const tokens: Token[] = [];

    while (!this.isEOF()) {
      let c = this.next();

      //replace tabs with spaces
      if (c === "\t") {
        c = " ";
      }

      switch (c) {
        case " ":
          break;
        case "\n":
          this.line++;
          break;

        case "@":
          tokens.push(new Token(TokenType.At, c, this.line));
          break;

        case ";":
          break;

        case "=":
          tokens.push(new Token(TokenType.Equals, c, this.line));
          break;

        case "{":
          tokens.push(new Token(TokenType.LBrace, c, this.line));
          break;
        case "}":
          tokens.push(new Token(TokenType.RBrace, c, this.line));
          break;

        case "(":
          tokens.push(new Token(TokenType.LParen, c, this.line));
          break;
        case ")":
          tokens.push(new Token(TokenType.RParen, c, this.line));
          break;

        case '"':
          let str = "";
          while (this.peek() !== '"') {
            str += this.next();
          }
          this.next();
          tokens.push(new Token(TokenType.String, str, this.line));
          break;

        default:
          if (isAlpha(c)) {
            let str = c;
            while (isAlphaNumeric(this.peek())) {
              str += this.next();
            }

            if (keywords.has(str)) {
              tokens.push(new Token(keywords.get(str)!, str, this.line));
            } else {
              tokens.push(new Token(TokenType.Identifier, str, this.line));
            }
          } else if (isDigit(c)) {
            let str = c;
            while (isDigit(this.peek())) {
              str += this.next();
            }

            if (this.peek() === ".") {
              str += this.next();
              while (isDigit(this.peek())) {
                str += this.next();
              }
            }

            tokens.push(
              new Token(TokenType.Number, parseFloat(str), this.line)
            );
          } else {
            throw new Error(`Unexpected character ${c} at line ${this.line}`);
          }
      }
    }

    return tokens;
  }

  private next(): string {
    return this.source[this.i++];
  }
  private peek(offset: number = 0): string {
    return this.source[this.i + offset];
  }
  isEOF(): boolean {
    return this.i >= this.source.length;
  }
}
