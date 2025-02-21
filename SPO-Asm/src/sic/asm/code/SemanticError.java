package sic.asm.code;

/**
 * Podporni razred za predmet Sistemska programska oprema.
 * @author jure
 */
@SuppressWarnings("serial")
public class SemanticError extends Exception {

	/**
	 * 
	 */
	public SemanticError(String msg) {
		super(msg + ".");
	}

}
