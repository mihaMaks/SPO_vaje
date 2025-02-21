package sic.asm.mnemonics;

//import sic.asm.code.Directive;
import sic.asm.code.Directive;
import sic.asm.code.Node;
import sic.asm.code.Storage;
import sic.asm.parsing.Parser;
import sic.asm.parsing.SyntaxError;


/**
 * Directive without operands.
 * Podporni razred za predmet Sistemska programska oprema.
 * @author jure
 */
public class MnemonicD extends Mnemonic {

	public MnemonicD(String mnemonic, int opcode, String hint, String desc) {
		super(mnemonic, opcode, hint, desc);
	}

	@Override
	public Node parse(Parser parser) throws SyntaxError {
		return new Directive(this, 0);
	}

}
