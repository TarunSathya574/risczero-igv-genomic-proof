# risczero-igv-genomic-proof
Using the Risczero protocol for zero knowledge proofs, we made a representation of genomic data using IGV standards and used the functionalities within Risczero to prove that key critical genomic data underlying some mutations either existed or did not exist within the data. 



 
# Problem Statement: 
Due to HIPAA and other privacy concerns with biomedical data, privacy preserving data sharing across research institutions and industries is vital for meeting strict constraints. Recent data leaks at 23andme where many genomic profiles of patients were exposed are proof of this urgent need to integrate privacy preserving cryptography in the era of genomic medicine and gene therapies, where data sharing is crucial for effective data analysis and AI based personalized therapeutic development using technologies such as Crispr Cas 9.  
# Solution 
We implemented a basic genomic proof showing that a patient had or didn't have a specific mutation for a rare genetic disease called cystic fibrosis using Risczero within a 24-48 hour timeframe using cryptographic techniques such as sha -256 hashing and ZK proof infrastructures in collaboration with the Risczero team at ZK Hack Montreal. 
# Execution Screenshots
Guest main file - proof logic
![image](https://github.com/user-attachments/assets/14728c5d-8b94-419d-9afe-85625553b534)

Host main file - proof generation/receipt
![image](https://github.com/user-attachments/assets/d198a4f4-b74f-4392-b318-1282934eaa45)

JSON File - patient's genome without Cystic Fibrosis 
![image](https://github.com/user-attachments/assets/2e3fbf56-2764-4cbe-b063-bb6764dcdbca)

Invalid Proof
![image](https://github.com/user-attachments/assets/cb51a006-a797-42e0-84d9-a2cd4918fb93)

JSON File - patient's genome with Cystic Fibrosis 
![image](https://github.com/user-attachments/assets/dbc81df1-081a-430f-82c7-e46313d07b2e)

Valid Proof - Chromosome 7 has the mutation sequence for Cystic Fibrosis
![image](https://github.com/user-attachments/assets/23c6ab2c-189d-4b38-86e8-547d7acc65c9)

# Video 


