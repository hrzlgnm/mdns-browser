/**
 * Workaround for thaw inputs wrapped by components `AutoComplete` or `Input`
 * having no possibility to set autocapitalize="none" which
 * prevents mobile inputs to auto-capitalize the first letter,
 * which is what we want
 */
window.addEventListener("TrunkApplicationStarted", function () {
    const inputs = document.querySelectorAll("input.thaw-input__input");
    for (const input of inputs) {
        input.setAttribute("autocapitalize", "none");
    }
});
