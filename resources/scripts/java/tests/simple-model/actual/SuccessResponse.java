import javax.annotation.Nonnull;

import java.util.List;

public class SuccessResponse {

    private @Nonnull Integer code;
    private List<List<String>> innerArray = new List<>();
    private InnerObject inner_object;
    private Integer message;
    private List<String> simpleArray = new List<>();

}
