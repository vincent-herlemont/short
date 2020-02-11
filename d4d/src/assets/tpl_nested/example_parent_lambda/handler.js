const handler = async (event, context) => {
    console.log("example parent lambda");
    console.log(event,context);
};

module.exports.handler = handler;