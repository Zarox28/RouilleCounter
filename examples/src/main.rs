rouille::rouille! {
  fonction principale() {
    soit mutable compteur = Quelque(0);

    tant soit Quelque(i) = compteur {
      si i == 10 {
        compteur = Rien;
      } sinon {
        affiche!("{}", i);
        compteur = Quelque(i + 1);
      }
    }
  }
}