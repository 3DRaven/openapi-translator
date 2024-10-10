import java.util.List;

public class Array {
    private List<String> items;
    public Array() {}
    public Array(List<String> items) {
        this.items = items;
    }
    public List<String> getArray() {
        return items;
    }
    public void setArray(List<String> items) {
        this.items = items;
    }
}