import java.util.List;

public class StringArraySchema {
    private List<String> items;
    public StringArraySchema() {}
    public StringArraySchema(List<String> items) {
        this.items = items;
    }
    public List<String> getStringArraySchema() {
        return items;
    }
    public void setStringArraySchema(List<String> items) {
        this.items = items;
    }
}