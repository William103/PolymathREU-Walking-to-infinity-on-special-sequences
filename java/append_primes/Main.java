import java.util.ArrayList;

public class Main
{
    public static void main(String[] args)
    {
        Util.init();

        Tree tree = new Tree(0, new ArrayList<Long>());
        for (int i = 0; i < 9; i++) {
            tree.step();
            Util.printList(tree.longestPath());
        }
    }
}
