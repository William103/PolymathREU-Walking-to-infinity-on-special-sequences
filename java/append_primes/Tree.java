import java.util.ArrayList;

class Tree
{
    private long value;
    private ArrayList<Tree> children;

    public Tree(long tree_value, ArrayList<Long> tree_children)
    {
        value = tree_value;
        children = new ArrayList<Tree>();
        for (long child : tree_children) {
            children.add(new Tree(child, new ArrayList<Long>()));
        }
    }

    public void step()
    {
        if (children.isEmpty()) {
            ArrayList<Long> xs = next(value);
            for (long val : xs) {
                children.add(new Tree(val, new ArrayList<Long>()));
            }
            return;
        }
        for (Tree child : children) {
            child.step();
        }
    }

    public ArrayList<Long> longestPath()
    {
        ArrayList<Long> retval = new ArrayList<Long>();
        if (children.isEmpty()) {
            retval.add(value);
            return retval;
        }
        int max_length = 0;
        for (Tree child : children) {
            ArrayList<Long> temp = child.longestPath();
            if (temp.size() > max_length) {
                max_length = temp.size();
                retval = temp;
            }
        }
        retval.add(0, value);
        return retval;
    }

    /* modify this function to change functionality */
    public ArrayList<Long> next(long val)
    {
        ArrayList<Long> new_xs = new ArrayList<Long>();
        long temp = val * 10;
        for (int i = 0; i < 10; i++) {
            long temp2 = temp + i;
            if (Util.isPrime(temp2)) {
                new_xs.add(temp2);
            }
        }
        return new_xs;
    }
}
