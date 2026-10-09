package run.codedock;

import java.util.Map;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
@RestController
public class DemoApplication {

    @GetMapping("/")
    public Map<String, String> hello() {
        return Map.of("message", "Hello from Codedock Java Spring Example!");
    }

    public static void main(String[] args) {
        SpringApplication.run(DemoApplication.class, args);
    }
}