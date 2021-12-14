package com.example;

import com.salesforce.functions.jvm.sdk.Context;
import com.salesforce.functions.jvm.sdk.InvocationEvent;
import com.salesforce.functions.jvm.sdk.SalesforceFunction;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ExampleFunction implements SalesforceFunction<String, String> {
    private final static Logger LOGGER = LoggerFactory.getLogger(ExampleFunction.class);

  @Override

  /*
  *
  * To test each section, you can just comment it out
  *
  * */
  public String apply(InvocationEvent<String> event, Context context) {
      /*
      *
      * Section with print statement at the beginning + multiple logging types
      *
      * */

      System.out.println("This is a test print statement at the beginning");

      LOGGER.info("This tests logging with INFO");
      LOGGER.warn("This tests logging with WARN");
      LOGGER.error("This tests logging with ERROR");
      LOGGER.debug("This tests logging with DEBUG");
      LOGGER.trace("This tests logging with TRACE");


      /*
       *
       * Section with print statement at the end
       *
       * */

//      LOGGER.info("This tests logging with INFO");
//      LOGGER.debug("This tests logging with DEBUG");
//      LOGGER.warn("This tests logging with WARN");
//      LOGGER.trace("This tests logging with TRACE");
//      LOGGER.error("This tests logging with ERROR");
//      LOGGER.debug("This tests logging with DEBUG");
//      System.out.println("This is a test print statement at the end");

      /*
      *
      * Section without print statements and multiple logging types
      *
      * */

//      LOGGER.info("This tests logging with INFO");
//      LOGGER.debug("This tests logging with DEBUG");
//      LOGGER.warn("This tests logging with WARN");
//      LOGGER.trace("This tests logging with TRACE");
//      LOGGER.error("This tests logging with ERROR");
//      LOGGER.debug("This tests logging with DEBUG");
//

      /*
       *
       * Section with print statement and multiple logging INFO
       *
       * */

      System.out.println("This is a test print statement for multiple INFO calls");

      LOGGER.info("This tests logging with INFO");
      LOGGER.info("This tests logging with INFO - 2nd call");
      LOGGER.info("This tests logging with INFO - 3rd call");
      LOGGER.info("This tests logging with INFO - 4th call");

      /*
       *
       * Section with print statement and multiple logging DEBUG
       *
       * */

      System.out.println("This is a test print statement for multiple INFO calls");

      LOGGER.debug("This tests logging with DEBUG");
      LOGGER.debug("This tests logging with DEBUG - 2nd call");
      LOGGER.debug("This tests logging with DEBUG - 3rd call");
      LOGGER.debug("This tests logging with DEBUG - 4th call");

      /*
       *
       * Section with print statement and multiple logging TRACE
       *
       * */

      System.out.println("This is a test print statement for multiple TRACE calls");

      LOGGER.trace("This tests logging with TRACE");
      LOGGER.trace("This tests logging with TRACE - 2nd call");
      LOGGER.trace("This tests logging with TRACE - 3rd call");
      LOGGER.trace("This tests logging with TRACE - 4th call");

      /*
       *
       * Section with print statement and multiple logging WARN
       *
       * */

      System.out.println("This is a test print statement for multiple WARN calls");

      LOGGER.warn("This tests logging with WARN");
      LOGGER.warn("This tests logging with WARN - 2nd call");
      LOGGER.warn("This tests logging with WARN - 3rd call");
      LOGGER.warn("This tests logging with WARN - 4th call");

      /*
       *
       * Section with print statement and multiple logging ERROR
       *
       * */

      System.out.println("This is a test print statement for multiple ERROR calls");

      LOGGER.error("This tests logging with ERROR");
      LOGGER.error("This tests logging with ERROR - 2nd call");
      LOGGER.error("This tests logging with ERROR - 3rd call");
      LOGGER.error("This tests logging with ERROR - 4th call");
    return new StringBuilder(event.getData()).reverse().toString();
  }
}
