/*
Copyright 2023 The bpfman Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

// Code generated by client-gen. DO NOT EDIT.

package v1alpha1

import (
	"context"
	"time"

	v1alpha1 "github.com/bpfman/bpfman/bpfman-operator/apis/v1alpha1"
	scheme "github.com/bpfman/bpfman/bpfman-operator/pkg/client/clientset/versioned/scheme"
	v1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	types "k8s.io/apimachinery/pkg/types"
	watch "k8s.io/apimachinery/pkg/watch"
	rest "k8s.io/client-go/rest"
)

// TracepointProgramsGetter has a method to return a TracepointProgramInterface.
// A group's client should implement this interface.
type TracepointProgramsGetter interface {
	TracepointPrograms() TracepointProgramInterface
}

// TracepointProgramInterface has methods to work with TracepointProgram resources.
type TracepointProgramInterface interface {
	Create(ctx context.Context, tracepointProgram *v1alpha1.TracepointProgram, opts v1.CreateOptions) (*v1alpha1.TracepointProgram, error)
	Update(ctx context.Context, tracepointProgram *v1alpha1.TracepointProgram, opts v1.UpdateOptions) (*v1alpha1.TracepointProgram, error)
	UpdateStatus(ctx context.Context, tracepointProgram *v1alpha1.TracepointProgram, opts v1.UpdateOptions) (*v1alpha1.TracepointProgram, error)
	Delete(ctx context.Context, name string, opts v1.DeleteOptions) error
	DeleteCollection(ctx context.Context, opts v1.DeleteOptions, listOpts v1.ListOptions) error
	Get(ctx context.Context, name string, opts v1.GetOptions) (*v1alpha1.TracepointProgram, error)
	List(ctx context.Context, opts v1.ListOptions) (*v1alpha1.TracepointProgramList, error)
	Watch(ctx context.Context, opts v1.ListOptions) (watch.Interface, error)
	Patch(ctx context.Context, name string, pt types.PatchType, data []byte, opts v1.PatchOptions, subresources ...string) (result *v1alpha1.TracepointProgram, err error)
	TracepointProgramExpansion
}

// tracepointPrograms implements TracepointProgramInterface
type tracepointPrograms struct {
	client rest.Interface
}

// newTracepointPrograms returns a TracepointPrograms
func newTracepointPrograms(c *BpfmanV1alpha1Client) *tracepointPrograms {
	return &tracepointPrograms{
		client: c.RESTClient(),
	}
}

// Get takes name of the tracepointProgram, and returns the corresponding tracepointProgram object, and an error if there is any.
func (c *tracepointPrograms) Get(ctx context.Context, name string, options v1.GetOptions) (result *v1alpha1.TracepointProgram, err error) {
	result = &v1alpha1.TracepointProgram{}
	err = c.client.Get().
		Resource("tracepointprograms").
		Name(name).
		VersionedParams(&options, scheme.ParameterCodec).
		Do(ctx).
		Into(result)
	return
}

// List takes label and field selectors, and returns the list of TracepointPrograms that match those selectors.
func (c *tracepointPrograms) List(ctx context.Context, opts v1.ListOptions) (result *v1alpha1.TracepointProgramList, err error) {
	var timeout time.Duration
	if opts.TimeoutSeconds != nil {
		timeout = time.Duration(*opts.TimeoutSeconds) * time.Second
	}
	result = &v1alpha1.TracepointProgramList{}
	err = c.client.Get().
		Resource("tracepointprograms").
		VersionedParams(&opts, scheme.ParameterCodec).
		Timeout(timeout).
		Do(ctx).
		Into(result)
	return
}

// Watch returns a watch.Interface that watches the requested tracepointPrograms.
func (c *tracepointPrograms) Watch(ctx context.Context, opts v1.ListOptions) (watch.Interface, error) {
	var timeout time.Duration
	if opts.TimeoutSeconds != nil {
		timeout = time.Duration(*opts.TimeoutSeconds) * time.Second
	}
	opts.Watch = true
	return c.client.Get().
		Resource("tracepointprograms").
		VersionedParams(&opts, scheme.ParameterCodec).
		Timeout(timeout).
		Watch(ctx)
}

// Create takes the representation of a tracepointProgram and creates it.  Returns the server's representation of the tracepointProgram, and an error, if there is any.
func (c *tracepointPrograms) Create(ctx context.Context, tracepointProgram *v1alpha1.TracepointProgram, opts v1.CreateOptions) (result *v1alpha1.TracepointProgram, err error) {
	result = &v1alpha1.TracepointProgram{}
	err = c.client.Post().
		Resource("tracepointprograms").
		VersionedParams(&opts, scheme.ParameterCodec).
		Body(tracepointProgram).
		Do(ctx).
		Into(result)
	return
}

// Update takes the representation of a tracepointProgram and updates it. Returns the server's representation of the tracepointProgram, and an error, if there is any.
func (c *tracepointPrograms) Update(ctx context.Context, tracepointProgram *v1alpha1.TracepointProgram, opts v1.UpdateOptions) (result *v1alpha1.TracepointProgram, err error) {
	result = &v1alpha1.TracepointProgram{}
	err = c.client.Put().
		Resource("tracepointprograms").
		Name(tracepointProgram.Name).
		VersionedParams(&opts, scheme.ParameterCodec).
		Body(tracepointProgram).
		Do(ctx).
		Into(result)
	return
}

// UpdateStatus was generated because the type contains a Status member.
// Add a +genclient:noStatus comment above the type to avoid generating UpdateStatus().
func (c *tracepointPrograms) UpdateStatus(ctx context.Context, tracepointProgram *v1alpha1.TracepointProgram, opts v1.UpdateOptions) (result *v1alpha1.TracepointProgram, err error) {
	result = &v1alpha1.TracepointProgram{}
	err = c.client.Put().
		Resource("tracepointprograms").
		Name(tracepointProgram.Name).
		SubResource("status").
		VersionedParams(&opts, scheme.ParameterCodec).
		Body(tracepointProgram).
		Do(ctx).
		Into(result)
	return
}

// Delete takes name of the tracepointProgram and deletes it. Returns an error if one occurs.
func (c *tracepointPrograms) Delete(ctx context.Context, name string, opts v1.DeleteOptions) error {
	return c.client.Delete().
		Resource("tracepointprograms").
		Name(name).
		Body(&opts).
		Do(ctx).
		Error()
}

// DeleteCollection deletes a collection of objects.
func (c *tracepointPrograms) DeleteCollection(ctx context.Context, opts v1.DeleteOptions, listOpts v1.ListOptions) error {
	var timeout time.Duration
	if listOpts.TimeoutSeconds != nil {
		timeout = time.Duration(*listOpts.TimeoutSeconds) * time.Second
	}
	return c.client.Delete().
		Resource("tracepointprograms").
		VersionedParams(&listOpts, scheme.ParameterCodec).
		Timeout(timeout).
		Body(&opts).
		Do(ctx).
		Error()
}

// Patch applies the patch and returns the patched tracepointProgram.
func (c *tracepointPrograms) Patch(ctx context.Context, name string, pt types.PatchType, data []byte, opts v1.PatchOptions, subresources ...string) (result *v1alpha1.TracepointProgram, err error) {
	result = &v1alpha1.TracepointProgram{}
	err = c.client.Patch(pt).
		Resource("tracepointprograms").
		Name(name).
		SubResource(subresources...).
		VersionedParams(&opts, scheme.ParameterCodec).
		Body(data).
		Do(ctx).
		Into(result)
	return
}
