package sic.asm.mnemonics;

import sic.asm.code.InstructionF2;
import sic.asm.code.Node;
import sic.asm.parsing.Parser;
import sic.asm.parsing.SyntaxError;

public class MnemonicF2rr extends Mnemonic {

    public MnemonicF2rr(String name, int opcode, String hint, String desc) {
        super(name, opcode, hint, desc);
    }

    @Override
    public Node parse(Parser parser) throws SyntaxError {
        int reg1=0;
        int reg2 =0;
        // fst and snd registers
        if (Character.isLetter(parser.lexer.peek())) {
            reg1 = parser.parseRegister();
        }
        parser.parseComma();
        if (Character.isLetter(parser.lexer.peek())) {
            reg2 = parser.parseRegister();
        }
        else
            throw new SyntaxError(String.format("Invalid character '%c", parser.lexer.peek()), parser.lexer.row, parser.lexer.col);
        return new InstructionF2(this, reg1, reg2);
    }

    @Override
    public String operandToString(Node instruction) {
        return instruction.toString();
    }
}
