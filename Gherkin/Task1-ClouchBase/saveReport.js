const axios = require('axios');


const apiEndpoint = 'http://localhost:3000/save-report'; 

axios.post(apiEndpoint)
  .then(response => {
    console.log('Data sent successfully');
  })
  .catch(error => {
    console.error('Error sending data:', error.message);
});

  
