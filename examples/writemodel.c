#include <iostream>

#include "ampl/ampl.h"

// This example shows different exception classes that
// can present themselves while writing a model, due
// to presolver detecting infeasible/trivial models
void makeAmplModel(ampl::AMPL &ampl, int numvars, bool unfeasible,
                   bool unbounded = false) {
  assert(!(unfeasible && unbounded));
  ampl.eval(
      "set varIndexes within Integers; set constraintIndexes within Integers;"
      "param varCosts{ varIndexes };"
      "param rightHandSides{ constraintIndexes };"
      "var x{ i in varIndexes } >= 0;"
      "maximize objectiveFunction : sum{ i in varIndexes } varCosts[i] * "
      "x[i];");

  ampl::DataFrame d(1, {"varIndexes", "varCosts"});
  for (int i = 0; i < numvars; i++) d.addRow(i, i);
  ampl.setData(d, "varIndexes");
  if (!unbounded) {
    ampl.eval("mainConstraints{j in constraintIndexes}:"
        "x[j] + x[j+1] <= rightHandSides[j];");
    ampl::DataFrame d2(1, {"constraintIndexes", "rightHandSides"});
    for (int i = 0; i < numvars - 1; i++) d2.addRow(i, unfeasible ? -1 : 1);
    ampl.setData(d2, "constraintIndexes");
  }
}



int main(int argc, char **argv) {
  ampl::AMPL a;
  bool caught = false;
  makeAmplModel(a, 10, true);

  try {  // Infeasible model
    a.write("gtest");
  } catch (const ampl::InfeasibilityException &e) {
    puts(e.what());
    caught = true;
  }
  if (!caught) throw std::runtime_error("caught is false");
  caught = false;

  try {  // Wrong filetype
    a.write("htest");
  } catch (const ampl::AMPLException &e) {
    puts(e.what());
    caught = true;
  }
  if (!caught) throw std::runtime_error("caught is false");
  caught = false;

  // This works
  a.setIntOption("presolve", 0);
  a.write("gtest", "cr");
  
  
  try {  // Trivial, expect PresolveException problem
    a.reset();
    a.setIntOption("presolve", 10);
    a.eval("var x; maximize z : x; c: x = 5;");
    a.write("gtest");
  } catch (const ampl::PresolveException &e) {
    puts(e.what());
    caught = true;
  }
  if (!caught) throw std::runtime_error("caught is false");

  return 0;
}