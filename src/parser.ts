import { TokenType, type Token } from "./lexer";

/*

    //parse tokens into AST example:
    [
        {type: TokenType.At, value: "@", line: 1},
        {type: TokenType.LBrace, value: "{", line: 2},
        {type: TokenType.Identifier, value: "name", line: 3},
        {type: TokenType.Equals, value: "=", line: 3},
        {type: TokenType.String, value: "hello world", line: 3},
        {type: TokenType.RBrace, value: "}", line: 4},
    ]
    {name: "hello world"}
*/
export class Parser {
  tokens: Token[];
  i: number;

  constructor(tokens: Token[]) {
    this.tokens = tokens;
    this.i = 0;
  }

  identifier(): string {
    const token = this.next();
    if (token.type !== TokenType.Identifier) {
      throw new Error(
        `Expected identifier but got ${token.type}  at line ${token.line}`
      );
    }
    return <string>token.value;
  }

  value(): any {
    switch (this.peek().type) {
      case TokenType.String:
        return this.string();
      case TokenType.Number:
        return this.number();
      case TokenType.Boolean:
        return this.boolean();
      case TokenType.Null:
        this.next();
        return null;
      default:
        if (this.match([TokenType.At])) {
          if (this.match([TokenType.LParen])) {
            return this.array();
          }
          if (this.match([TokenType.LBrace])) {
            return this.object();
          }
        }

        throw new Error(
          `Expected value but got ${this.peek().type} at line ${
            this.peek().line
          }`
        );
    }
  }

  string(): string {
    const token = this.next();
    if (token.type !== TokenType.String) {
      throw new Error(
        `Expected string but got ${token.type} at line ${token.line}`
      );
    }
    return <string>token.value;
  }
  number(): number {
    const token = this.next();
    if (token.type !== TokenType.Number) {
      throw new Error(
        `Expected number but got ${token.type} at line ${token.line}`
      );
    }
    return <number>token.value;
  }
  boolean(): boolean {
    const token = this.next();
    if (token.type !== TokenType.Boolean) {
      throw new Error(
        `Expected boolean but got ${token.type} at line ${token.line}`
      );
    }

    switch (token.value) {
      case "$true":
        return true;
      case "$false":
        return false;
      default:
        throw new Error(
          `Expected boolean but got ${token.type} at line ${token.line}`
        );
    }
  }

  equals(): void {
    const token = this.next();
    if (token.type !== TokenType.Equals) {
      throw new Error(
        `Expected equals but got ${token.type} at line ${token.line}`
      );
    }
  }

  array(): any[] {
    const arr: any[] = [];

    //empty array
    if (this.peek().type === TokenType.RParen) {
      this.next();
      return arr;
    }

    while (!this.check(TokenType.RParen) && !this.isEOF()) {
      const value = this.value();
      arr.push(value);
    }

    this.next(); //RParen

    return arr;
  }

  object(): any {
    let obj: any = {};

    //empty object
    if (this.peek().type === TokenType.RBrace) {
      this.next();
      return obj;
    }

    while (!this.check(TokenType.RBrace) && !this.isEOF()) {
      const key = this.identifier();
      this.equals();
      const value = this.value();
      obj[key] = value;
    }

    this.next(); //RBrace

    return obj;
  }

  private next(): Token {
    return this.tokens[this.i++];
  }
  private peek(offset: number = 0): Token {
    return this.tokens[this.i + offset];
  }
  private isEOF(): boolean {
    return this.i >= this.tokens.length;
  }

  private check(type: TokenType): boolean {
    if (this.isEOF()) return false;
    return this.peek().type === type;
  }
  private match(types: TokenType[]): boolean {
    for (const type of types) {
      if (this.check(type)) {
        this.next();
        return true;
      }
    }
    return false;
  }
}
