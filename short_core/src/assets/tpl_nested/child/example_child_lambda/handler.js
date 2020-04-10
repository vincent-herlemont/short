const handler = async (event, context) => {
    console.log("example child lambda");
    console.log(event,context);
};

module.exports.handler = handler;