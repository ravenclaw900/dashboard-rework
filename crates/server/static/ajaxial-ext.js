Ajaxial.swapStrategies.idiomorphouter = (target, fragment) => {
    Idiomorph.morph(target, fragment);
}

Ajaxial.swapStrategies.idiomorphinner = (target, fragment) => {
    Idiomorph.morph(target, fragment, { morphStyle: "innerHTML" });
}

Ajaxial.swapStrategies[Ajaxial.default] = "idiomorphinner";