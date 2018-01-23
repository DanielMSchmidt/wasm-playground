import { add } from './add.rs';
import { request } from './src/request.rs';

console.log('add:', add(2, 3));
console.log('request', request());