function renderComment(comment: string) {
    const text = document.createTextNode(comment);
    document.getElementById("comments").appendChild(text);
}
