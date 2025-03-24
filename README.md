### 1. Clone the Llama.cpp repository
```bash
git clone https://github.com/ggerganov/llama.cpp.git  
```

### 2. Create build directory
```bash
cd llama.cpp && mkdir build && cd build
```

### 3. Compile the code
```bash
cmake .. -DLLAMA_NATIVE=ON
cmake --build . --config Release
```

### 4. Run python server
```bash
uvicorn scripts.embed_server:app --reload --port 8000
```