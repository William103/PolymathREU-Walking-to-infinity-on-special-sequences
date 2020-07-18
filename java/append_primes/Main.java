import java.util.ArrayList;

public class Main
{
    public static void main(String[] args)
    {
        Util.init();

        ArrayList<Long> vals = new ArrayList<Long>();
        vals.add(2l);
        vals.add(3l);
        vals.add(5l);
        vals.add(7l);

        Tree tree = new Tree(0, vals);
        for (int i = 0; i < 20; i++) {
            tree.step();
            Util.printList(tree.longestPath());
        }
    }
}
