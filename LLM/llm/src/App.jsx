import React, { useState, useRef, useEffect } from "react";
import axios from "axios";

const App = () => {
  const [userInput, setUserInput] = useState("");
  const [history, setHistory] = useState([]); 
  const [loading, setLoading] = useState(false); 
  const apiKey = import.meta.env.VITE_API_URL;
  const chatEndRef = useRef(null);

  useEffect(() => {
    chatEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [history]);

  const handleSendMessage = async () => {
    if (!userInput.trim()) return;

    setLoading(true); 
    const apiUrl = "https://openrouter.ai/api/v1/chat/completions";

    const messages = [
      { role: "system", content: "You are a helpful AI assistant. Remember user details when mentioned." },
      ...history.map(({ user, bot }) => [{ role: "user", content: user }, { role: "assistant", content: bot }]).flat(),
      { role: "user", content: userInput }
    ];

    const payload = {
      model: "gpt-3.5-turbo",
      messages,
      max_tokens: 1000,
    };

    try {
      const res = await axios.post(apiUrl, payload, {
        headers: {
          Authorization: `Bearer ${apiKey}`,
          "Content-Type": "application/json",
        },
      });


      console.log(res.data);
      
      const botResponse = res.data.choices?.[0]?.message?.content || "No response from AI.";

      setHistory((prevHistory) => [
        ...prevHistory,
        { user: userInput, bot: botResponse },
      ]);
    } catch (error) {
      console.error("Error:", error);
      setHistory((prevHistory) => [
        ...prevHistory,
        { user: userInput, bot: "Error fetching response. Try again." },
      ]);
    }

    setUserInput(""); 
    setLoading(false); 
  };

  const handleKeyPress = (e) => {
    if (e.key === "Enter") {
      handleSendMessage();
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gradient-to-r from-blue-400 to-indigo-600 px-4">
      <h1 className="text-4xl font-bold text-white mb-6">AI Chatbot</h1>

      <div className="w-full max-w-2xl h-[70vh] md:h-[500px] p-4 bg-white border border-gray-300 rounded-xl shadow-lg overflow-y-auto flex flex-col">
        {history.length === 0 ? (
          <p className="text-gray-400 text-center text-lg mt-auto mb-2">Ask me anything...</p>
        ) : (
          <div className="flex flex-col space-y-3">
            {history.map((entry, index) => (
              <div key={index} className="flex flex-col">
                <div className="self-end bg-blue-500 text-white px-4 py-3 rounded-lg max-w-[75%] shadow">
                  {entry.user}
                </div>
                <div className="self-start bg-gray-200 text-gray-900 px-4 py-3 rounded-lg max-w-[75%] shadow mt-2">
                  {entry.bot}
                </div>
              </div>
            ))}
          </div>
        )}
        <div ref={chatEndRef} />
      </div>

      <div className="flex w-full max-w-2xl mt-4 space-x-2">
        <input
          type="text"
          value={userInput}
          onChange={(e) => setUserInput(e.target.value)}
          onKeyDown={handleKeyPress}
          placeholder="Type a message..."
          className="flex-1 px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-lg"
          disabled={loading}
        />
        <button
          onClick={handleSendMessage}
          className="px-6 py-3 bg-white text-blue-500 font-bold rounded-lg hover:bg-blue-50 transition duration-300 flex items-center justify-center"
          disabled={loading}
        >
          {loading ? (
            <svg className="animate-spin h-5 w-5 mr-2 border-t-2 border-blue-500 rounded-full" viewBox="0 0 24 24"></svg>
          ) : (
            "Send"
          )}
        </button>
      </div>
    </div>
  );
};

export default App;
