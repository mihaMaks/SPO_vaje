package sic.asm.mnemonics;

import sic.asm.code.Code;
import sic.asm.code.InstructionF2;
import sic.asm.code.InstructionF3;
import sic.asm.code.Node;
import sic.asm.parsing.Parser;
import sic.asm.parsing.SyntaxError;

public class MnemonicF2rn extends Mnemonic {
    public MnemonicF2rn(String name, int opcode, String hint, String desc) {
        super(name, opcode, hint, desc);
    }

    @Override
    public Node parse(Parser parser) throws SyntaxError {
        int n;
        int reg1=0;
        // number
        if (Character.isLetter(parser.lexer.peek()))
            reg1 = parser.parseRegister();
        parser.parseComma();
        if (Character.isDigit(parser.lexer.peek()))
            n = parser.parseNumber(0, Code.MAX_WORD);
        else
            throw new SyntaxError(String.format("Invalid character '%c", parser.lexer.peek()), parser.lexer.row, parser.lexer.col);
        return new InstructionF2(this, reg1, n);

    }

    @Override
    public String operandToString(Node instruction) {
        return instruction.toString();
    }

}
