
# 🚀 Rust Projects Collection  
Welcome to my **Rust Projects Collection**! This repository showcases three beginner-to-intermediate level Rust projects, each exploring different use cases and concepts.  

---

## 📂 Projects Overview  

1. **Chat Application (CLI-based)**  
   A command-line interface (CLI) chat application with server-client architecture, enabling real-time communication.  

2. **HTTP Server**  
   A lightweight HTTP server to serve static files like HTML, CSS, and JS with user-specified file paths.  

3. **Web Scraper**  
   A basic web scraper that extracts key information, such as title, meta description, links, and images, from a given website URL.  

---

## 🔍 Project Details  

### 1️⃣ Chat Application (CLI-based)  

**Description:**  
This CLI-based chat application allows users to connect to a central server, set a username, and chat in real time.  

**Features:**  
- Unique username support  
- Server-client communication via TCP  
- User-friendly CLI interface  
- Multi-client support  
- Error handling for invalid inputs  

**How to Run:**  
1. Clone the repository:  
   ```bash
   git clone https://github.com/Jain-Diablo/Rust_Projects.git
   cd rust-projects/chat_application
   ```  
2. Run the server:  
   ```bash
   cargo run --bin server
   ```
3. Open another terminal and connect a client:  
   ```bash
   cargo run --bin client
   ```  

   **Note:** You can connect as many clients as you want.  

---

### 2️⃣ HTTP Server  

**Description:**  
This HTTP server serves static files like HTML, CSS, or JS from a specified directory over a local or network-accessible address.  

**Features:**  
- Easy file hosting by specifying a directory  
- Supports multiple MIME types  
- Lightweight and efficient  

**How to Run:**  
1. Navigate to the project directory:  
   ```bash
   cd rust-projects/http_server
   ```  
2. Build and run the server:  
   ```bash
   cargo run
   ```  
3. Open the `public` folder and locate the `index.html` file.  
4. Open the `index.html` file in a browser. You will find an input field where you can enter the directory path.  
5. Access the hosted files via the provided URL in your browser.  

---

### 3️⃣ Web Scraper  

**Description:**  
A simple web scraper that extracts the following information from any URL:  
- Page title  
- Headings  
- Meta description  
- Links  
- Images  

**Features:**  
- Clean extraction of key elements  
- Handles edge cases for missing metadata  
- User-friendly CLI interaction  

**How to Run:**  
1. Navigate to the project directory:  
   ```bash
   cd rust-projects/web_scraper
   ```  
2. Build and run the scraper:  
   ```bash
   cargo run
   ```  
3. Open the `index.html` file in a browser.  
4. Enter the website URL in the input field.  
5. The details will be displayed in a tabular format.  

---

## 🛠 Prerequisites  

- **Rust Toolchain:** Make sure you have Rust and Cargo installed. [Install Rust](https://www.rust-lang.org/tools/install)  
- **Internet Connection:** Required for web scraping and the multi-client chat app.  

---

## 🤝 Contributing  

Contributions, issues, and feature requests are welcome! Feel free to open a pull request or submit an issue to suggest improvements.  

---

### Happy Rusting! 🦀  

-
