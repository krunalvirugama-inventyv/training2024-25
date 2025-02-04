const mongoose = require('mongoose');

const reportSchema = new mongoose.Schema({
    report: Object,
    createdAt: {
        type: Date,
        default: Date.now
    }
});


const Report = mongoose.model('Report', reportSchema);

module.exports = Report;