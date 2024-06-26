import React, { useState } from 'react';

const PaperTemplate = () => {
  const [userName, setUserName] = useState('');
  const [paperName, setPaperName] = useState('');
  const [wrongAnswerList, setWrongAnswerList] = useState([]);
  const [homeworkList, setHomeworkList] = useState([]);
  const [pictureUrls, setPictureUrls] = useState([]);
  const prefix = "/home/yaoxiao/mydoc/rust-mongo";
  const handleSubmit = async (e) => {
    e.preventDefault();

    // Create a data object with the form values

    const data = {
      user_name: userName,
      paper_name: paperName,
      wrong_answer_list: wrongAnswerList.map(Number),
      homework_list: homeworkList.map(Number)
    };

    try {
      // Send a POST request to the backend
      const response = await fetch('http://localhost:8080/api/submitForm', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      });

      if (response.ok) {
        // Parse the response and extract the list of picture URLs
        const picture_urls = await response.json();
        setPictureUrls(picture_urls);
      } else {
        console.error('Failed to submit the form');
      }
    } catch (error) {
      console.error('Error:', error);
    }
  };

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <label>
          User Name:
          <input
            type="text"
            value={userName}
            onChange={(e) => setUserName(e.target.value)}
          />
        </label>
        <br />
        <label>
          Paper Name:
          <input
            type="text"
            value={paperName}
            onChange={(e) => setPaperName(e.target.value)}
          />
        </label>
        <br />
        <label>
          Wrong Answer List:
          <input
            type="text"
            value={wrongAnswerList}
            onChange={(e) => setWrongAnswerList(e.target.value.split(','))}
          />
        </label>
        <br />
        <label>
          Homework List:
          <input
            type="text"
            value={homeworkList}
            onChange={(e) => setHomeworkList(e.target.value.split(','))}
          />
        </label>
        <br />
        <button type="submit">Submit</button>
      </form>
      {
        pictureUrls.length === 0 && (
          <p>No pictures to display</p>
        )
      }
      {pictureUrls.length > 0 && (
        <div>
          <h2>Wrong Answer Pictures:</h2>
          <ol>
            {pictureUrls.map((url, index) => (
              <li key={url.id} style={{ display: 'flex', alignItems: 'flex-start' }}>
                <span style={{ marginRight: '10px' }}>{index + 1}.</span>
                {url.q_url.map((q) => (
                  <img src={q.replace(prefix, '.')} alt="Wrong Answer" />
                ))}
              </li>
            ))}
        </ol>
        </div>
      )}
    </div>
  );
};

export default PaperTemplate;
