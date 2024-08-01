## MV - MarkdownViewer

A markdown viewer tool implemented in Rust.

Markdown has a simple syntax and a beautiful visualization. Therefore, markdown has become the de facto standard for technical API documentation and memos. However, existing markdown tools are not so good in two aspects:

1. The editing capability is not necessary, and even harmful in some scenarios. For some non-technical users, there is no dedicated markdown viewer. Many markdown software are designed to have both editing and rendering functions, however, the editing function is not necessary for non-technical users. Furthermore, when managing plenty of markdown files, the editing window squeezes the visualization area.

2. Lightweight and efficient visualization tools are still thirsty. Although some tools focus on visualization functions, they are based on C# or JavaScript, resulting in large package sizes and low running efficiency. For example, the size of MarkdownViewer is 43.3MB and the size of mdview is 76.6MB. There is no tool like Sumatra yet.

I argue that a pure rust-based visualization tool is crucial. This is because Rust has been proven to have good running performance while taking up very few resources.

My tool is called MV (MarkdownViewer).

Plan:
Coming soon...