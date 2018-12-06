class Hamming {

    private String left;
    private String right;

    public Hamming(String leftStrand, String rightStrand) {
	if (leftStrand.length() != rightStrand.length())
	    throw new IllegalArgumentException("leftStrand and rightStrand must be of equal length.");
    	left = leftStrand;
	right = rightStrand;
    }

    public int getHammingDistance() {
	int dist = 0;
	for (int i = 0; i < left.length(); i++){
	    char l = left.charAt(i);
	    char r = right.charAt(i);
	    if (l != r)
		dist += 1;
	}
	return dist;
    }

}
