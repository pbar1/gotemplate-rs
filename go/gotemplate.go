package main

import (
	"C"

	"bytes"
	"encoding/json"
	"text/template"

	"github.com/Masterminds/sprig"
)

const defaultTemplateName = "base"

func main() {}

//export Render
func Render(template_body, json_data string) *C.char {
	// Deserialize data object from JSON
	data := new(interface{})
	if err := json.Unmarshal([]byte(json_data), data); err != nil {
		return C.CString("error unmarshalling json data: " + err.Error())
	}

	// Add more functions to template scope
	// Inspired by Helm: https://github.com/helm/helm/blob/master/pkg/engine/funcs.go
	funcMap := sprig.TxtFuncMap()

	// Create Go template from passed body
	tpl, err := template.New(defaultTemplateName).Funcs(funcMap).Parse(template_body)
	if err != nil {
		return C.CString("error parsing go template: " + err.Error())
	}

	// Render template using data object
	buf := new(bytes.Buffer)
	if err := tpl.Execute(buf, data); err != nil {
		return C.CString("error rendering go template: " + err.Error())
	}

	return C.CString(buf.String())
}
