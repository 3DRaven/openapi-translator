import org.springframework.beans.factory.annotation.Value;
import org.springframework.beans.factory.annotation.Autowired;
import java.util.List;

public class PagingSimplifiedTrackObject {


    private  String another_field;
    @Value
    @Autowired

    private  String some_field;
    private  List<CustomCode> items = new List<>();

}
