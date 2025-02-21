package sic.asm.parsing;

/**
 * Podporni razred za predmet Sistemska programska oprema.
 * @author jure
 */
@SuppressWarnings("serial")
public class SyntaxError extends Exception {
	int row;
	int col;

	public SyntaxError(String msg, int row, int col) {
		super(msg);
		this.row = row;
		this.col = col;
	}

	@Override
	public String toString() {
        String head = "Syntax error at " + Integer.toString(this.row) + ", " + Integer.toString(this.col);
        String message = this.getLocalizedMessage();
        return ((message != null) ? (head + ": " + message) : head) + ".";
	}
}