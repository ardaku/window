fn main() {
    res::generate(&[
        res::shader("color").transform().gradient(),
        res::shader("graphic").transform().graphic().depth(),
        res::shader("blend").transform().graphic().depth().blend(),
    ]);
}
