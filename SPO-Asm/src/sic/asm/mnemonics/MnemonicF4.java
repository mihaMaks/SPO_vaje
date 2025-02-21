package sic.asm.mnemonics;

import sic.asm.code.Code;
import sic.asm.code.InstructionF3;
import sic.asm.code.InstructionF4;
import sic.asm.code.Node;
import sic.asm.parsing.Parser;
import sic.asm.parsing.SyntaxError;

public class MnemonicF4 extends Mnemonic {
    public String type = "";
    public MnemonicF4(String name, int opcode, String hint, String desc) {
        super(name, opcode, hint, desc);
        this.name = "+" + this.name;
    }

    @Override
    public Node parse(Parser parser) throws SyntaxError {

        if (parser.lexer.peek() == '#' || parser.lexer.peek() == '@'){
            type = Character.toString(parser.lexer.peek()); parser.lexer.advance();
        }
        // number
        if (Character.isDigit(parser.lexer.peek()))
            return new InstructionF4(this, type + parser.parseNumber(0, Code.MAX_WORD));
            // symbol
        else if (Character.isLetter(parser.lexer.peek()))
            return new InstructionF4(this, type + parser.parseSymbol());

            // otherwise: error
        else
            throw new SyntaxError(String.format("Invalid character '%c", parser.lexer.peek()), parser.lexer.row, parser.lexer.col);
    }
    @Override
    public String operandToString(Node instruction) {
        return instruction.toString();
    }
}
