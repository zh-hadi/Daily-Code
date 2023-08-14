// global variable 
var a = 20; // a=20
console.log(a)


var ar = [10, 20] // create array
var ar2 = new Array(5) // create array
var arr2 = [];
ar2[0] = 40;
console.log(ar, ar2, arr2)
// add two array 
const arrr = [...ar, ...ar2]
console.log(arrr)
// destructuring operator 
const [x, y] = ar
console.log(x, y)

// create object 
const ob = {
    a: 20,
    b: 30
}
const ob2 =  Object.create({
    x: 20,
    y: 30
})

function CreatObj(name, age){
    this.name = name;
    this.age = age;
}
const ob3 =new CreatObj("hadi", 21);

// class object create
class Ob{
    constructor(firstName, lastName){
        this.firstName = firstName;
        this.lastName = lastName;
    }
}
const ob4 = new Ob("Hadi", "uzzaman")
console.log(ob, ob2, ob3, ob4)

// check property for 
console.log(ob4.hasOwnProperty("firstName"))
console.log("firstName" in ob4)

// array check
console.log(Array.isArray(ob4))
console.log(Array.isArray(ar))

