package append_primes;

import java.util.ArrayList;

public class Main
{
    public static void main(String[] args)
    {
        Util.init();

        /* By changing the args[0] we can modify which prime we want to be the 
         * first in the row. Same for the number of iteration.
         */
        Tree tree = new Tree(Integer.parseInt(args[0]), new ArrayList<Long>());
        for (int i = 0; i < Integer.parseInt(args[1]); i++) {
            tree.step();
            Util.printList(tree.longestPath());
        }
    }
}
