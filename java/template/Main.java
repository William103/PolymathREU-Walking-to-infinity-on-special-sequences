import java.util.ArrayList;

/* Here is where you can tweak the starting conditions */
public class Main
{
    public static void main(String[] args)
    {
        Util.init();

        /* here we create a new list with value 0 and no children and check 20
         * iterations, feel free to tweak this to your heart's content
         */
        Tree tree = new Tree(0, new ArrayList<Long>());
        for (int i = 0; i < 20; i++) {
            tree.step();
            Util.printList(tree.longestPath());
        }
    }
}
