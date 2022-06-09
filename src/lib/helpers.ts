export function getFormData(form){
    const formData = new FormData(form);

    const data: any = {};
    for (let field of formData) {
      const [key, value] = field;
      data[key] = value;
    }

    return data;
}