package sic.asm.mnemonics;

import sic.asm.code.Code;
import sic.asm.code.Directive;
import sic.asm.code.Node;
import sic.asm.code.Opcode;
import sic.asm.parsing.Parser;
import sic.asm.parsing.SyntaxError;


/**
 * Directive with one numeric operand.
 * Podporni razred za predmet Sistemska programska oprema.
 * @author jure
 */
public class MnemonicDn extends Mnemonic {

	public MnemonicDn(String mnemonic, int opcode, String hint, String desc) {
		super(mnemonic, opcode, hint, desc);
	}
	
	@Override
	public Node parse(Parser parser) throws SyntaxError {
		// number
		if (Character.isDigit(parser.lexer.peek()))
			return new Directive(this, parser.parseNumber(0, Code.MAX_ADDR));
		//
		else if (this.opcode == Directive.EQU)
			return new Directive(this, "");
			// symbol
		else if (Character.isLetter(parser.lexer.peek()))
			return new Directive(this, parser.parseSymbol());
		else
			throw new SyntaxError(String.format("Invalid character '%c", parser.lexer.peek()), parser.lexer.row, parser.lexer.col);
	}

//	@Override
//	public String operandToString(Node instruction) {
//		Directive i = ((Directive)instruction);
//		return i.symbol != null ? i.symbol : Integer.toString(i.value);
//	}

}
