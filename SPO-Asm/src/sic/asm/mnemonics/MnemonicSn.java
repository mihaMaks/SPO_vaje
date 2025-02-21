package sic.asm.mnemonics;

import sic.asm.code.Code;
import sic.asm.code.InstructionF3;
import sic.asm.code.Node;
import sic.asm.code.Storage;
import sic.asm.parsing.Parser;
import sic.asm.parsing.SyntaxError;

public class MnemonicSn extends Mnemonic {
    public MnemonicSn(String name, int opcode, String hint, String desc) {
        super(name, opcode, hint, desc);
    }

    @Override
    public Storage parse(Parser parser) throws SyntaxError {
        if (Character.isDigit(parser.lexer.peek()))
            return new Storage(this, parser.parseNumber(0, Code.MAX_ADDR));
        else
            throw new SyntaxError(String.format("Invalid character '%c", parser.lexer.peek()), parser.lexer.row, parser.lexer.col);
    }

    @Override
    public String operandToString(Node instruction) {
        return instruction.toString();
    }
}
