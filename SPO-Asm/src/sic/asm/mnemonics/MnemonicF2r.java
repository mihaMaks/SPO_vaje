package sic.asm.mnemonics;

import sic.asm.code.InstructionF2;
import sic.asm.code.Node;
import sic.asm.parsing.Parser;
import sic.asm.parsing.SyntaxError;

public class MnemonicF2r extends Mnemonic {
    public MnemonicF2r(String name, int opcode, String hint, String desc) {
        super(name, opcode, hint, desc);
    }

    @Override
    public Node parse(Parser parser) throws SyntaxError {
        // number
        if (Character.isLetter(parser.lexer.peek()))
            return new InstructionF2(this, parser.parseRegister());
        else
            throw new SyntaxError(String.format("Invalid character '%c", parser.lexer.peek()), parser.lexer.row, parser.lexer.col);
    }
    @Override
    public String toString() {
        return super.toString();
    }
}
