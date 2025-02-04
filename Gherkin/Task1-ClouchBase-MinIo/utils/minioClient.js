// utils/minioClient.js
const Minio = require('minio');

const minioClient = new Minio.Client({
    endPoint: 'localhost',
    port: 9000,
    useSSL: false,
    accessKey: 'minioadmin',
    secretKey: 'minioadmin'
});

const ensureBucketExists = async (bucketName) => {
    const exists = await minioClient.bucketExists(bucketName);
    if (!exists) {
        await minioClient.makeBucket(bucketName);
    }
};

const uploadToMinio = async (bucketName, objectName, filePath) => {
    await ensureBucketExists(bucketName);
    return minioClient.fPutObject(bucketName, objectName, filePath);
};

module.exports = {
    uploadToMinio
};